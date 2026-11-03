use bevy::prelude::*;

/// デフォルト（テスト・フォールバック用）マップサイズ定義
pub const DEFAULT_MAP_WIDTH: i32 = 28;
pub const DEFAULT_MAP_HEIGHT: i32 = 16;

pub const SQRT_3: f32 = 1.7320508;

/// 後方互換用エイリアス（動的マップサイズ時は MapGrid / MapConfig の幅を参照してください）
pub const MAP_WIDTH: i32 = DEFAULT_MAP_WIDTH;
pub const MAP_HEIGHT: i32 = DEFAULT_MAP_HEIGHT;

/// 軸座標系 (Axial Coordinates: q, r)
/// Pointy-topped (上が尖った六角形) を採用
/// 横方向のオフセット座標 (col, row):
/// row = r
/// col = q + (r - (r & 1)) / 2
/// 逆変換:
/// q = col - (row - (row & 1)) / 2
/// r = row
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32,
}

impl HexCoord {
    pub const fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    /// 横ループの正規化を行わない（ラップ前のcolを指定した）HexCoordを作成
    pub fn from_col_row_unwrapped(col: i32, row: i32) -> Self {
        let q = col - (row - (row & 1)) / 2;
        Self { q, r: row }
    }

    /// (col, row) のオフセット座標からHexCoordを作成（指定マップ幅でラップ）
    pub fn from_col_row_with_width(col: i32, row: i32, map_width: i32) -> Self {
        Self::from_col_row_unwrapped(col.rem_euclid(map_width), row)
    }

    /// (col, row) のオフセット座標からHexCoordを作成（デフォルト幅、テスト用）
    #[cfg(test)]
    pub fn from_col_row(col: i32, row: i32) -> Self {
        Self::from_col_row_with_width(col, row, MAP_WIDTH)
    }

    /// 横ループの正規化を行わない (col, row) のオフセット座標を取得
    pub fn to_col_row_unwrapped(self) -> (i32, i32) {
        let row = self.r;
        let col = self.q + (row - (row & 1)) / 2;
        (col, row)
    }

    /// (col, row) のオフセット座標を取得 (col は 0..map_width-1 に正規化)
    pub fn to_col_row_with_width(self, map_width: i32) -> (i32, i32) {
        let (col, row) = self.to_col_row_unwrapped();
        (col.rem_euclid(map_width), row)
    }

    /// (col, row) のオフセット座標を取得 (col は 0..MAP_WIDTH-1 に正規化、デフォルト幅、テスト用)
    #[cfg(test)]
    pub fn to_col_row(self) -> (i32, i32) {
        self.to_col_row_with_width(MAP_WIDTH)
    }

    /// 横ループを考慮して正規化された HexCoord を取得（指定マップ幅）
    pub fn wrapped_with_width(self, map_width: i32) -> Self {
        let (col, row) = self.to_col_row_with_width(map_width);
        Self::from_col_row_with_width(col, row, map_width)
    }

    /// 立方体座標の s 成分 (q + r + s = 0)
    #[inline]
    #[allow(dead_code)]
    pub const fn s(self) -> i32 {
        -self.q - self.r
    }

    /// 2つのヘックス間の距離（タイル数、横ループ考慮、指定マップ幅）
    pub fn distance_with_width(self, other: HexCoord, map_width: i32) -> i32 {
        let (c2, r2) = other.to_col_row_with_width(map_width);

        let base_q = c2 - (r2 - (r2 & 1)) / 2;
        let self_s = -self.q - self.r;
        let dr = (self.r - r2).abs();

        let mut min_dist = (self.q - base_q).abs() + (self_s - (-base_q - r2)).abs();

        let q_pos = base_q + map_width;
        let d_pos = (self.q - q_pos).abs() + (self_s - (-q_pos - r2)).abs();
        if d_pos < min_dist {
            min_dist = d_pos;
        }

        let q_neg = base_q - map_width;
        let d_neg = (self.q - q_neg).abs() + (self_s - (-q_neg - r2)).abs();
        if d_neg < min_dist {
            min_dist = d_neg;
        }

        (min_dist + dr) / 2
    }

    /// 2つのヘックス間の距離（タイル数、横ループ考慮、テスト用）
    #[cfg(test)]
    pub fn distance(self, other: HexCoord) -> i32 {
        self.distance_with_width(other, MAP_WIDTH)
    }

    /// 隣接する6つのヘックス座標を取得（横ループ考慮でラップ、指定マップ幅）
    pub fn neighbors_with_width(self, map_width: i32) -> [HexCoord; 6] {
        const DIRECTIONS: [(i32, i32); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];

        let mut res = [self; 6];
        for i in 0..6 {
            let (dq, dr) = DIRECTIONS[i];
            let raw_coord = HexCoord::new(self.q + dq, self.r + dr);
            res[i] = raw_coord.wrapped_with_width(map_width);
        }
        res
    }

    /// 隣接する6つのヘックス座標を取得（横ループ考慮でラップ、テスト用）
    #[cfg(test)]
    pub fn neighbors(self) -> [HexCoord; 6] {
        self.neighbors_with_width(MAP_WIDTH)
    }

    /// ヘックス座標から 3D ワールド座標 (X, 0.0, Z) を計算
    /// pointy-topped hex の場合:
    /// x = size * sqrt(3) * (q + r / 2)
    /// z = size * 3/2 * r
    pub fn to_world_pos(self, hex_radius: f32) -> Vec3 {
        let x = hex_radius * SQRT_3 * (self.q as f32 + self.r as f32 / 2.0);
        let z = hex_radius * 1.5 * (self.r as f32);
        Vec3::new(x, 0.0, z)
    }

    /// 3D ワールド座標 (X, Z) から最も近い HexCoord を逆算（Pointy-topped、指定マップ幅）
    pub fn from_world_pos_with_width(pos: Vec3, hex_radius: f32, map_width: i32) -> Self {
        let q_frac = (SQRT_3 / 3.0 * pos.x - 1.0 / 3.0 * pos.z) / hex_radius;
        let r_frac = (2.0 / 3.0 * pos.z) / hex_radius;
        let s_frac = -q_frac - r_frac;

        let mut q = q_frac.round() as i32;
        let mut r = r_frac.round() as i32;
        let s = s_frac.round() as i32;

        let q_diff = (q as f32 - q_frac).abs();
        let r_diff = (r as f32 - r_frac).abs();
        let s_diff = (s as f32 - s_frac).abs();

        if q_diff > r_diff && q_diff > s_diff {
            q = -r - s;
        } else if r_diff > s_diff {
            r = -q - s;
        }

        HexCoord::new(q, r).wrapped_with_width(map_width)
    }

    /// 3D ワールド座標 (X, Z) から最も近い HexCoord を逆算（Pointy-topped）
    /// 横ループを考慮して正規化
    pub fn from_world_pos(pos: Vec3, hex_radius: f32) -> Self {
        Self::from_world_pos_with_width(pos, hex_radius, MAP_WIDTH)
    }
}

/// 指定マップ幅における1周分のワールド横幅（X軸メートル）
pub fn map_world_width_with_width(hex_radius: f32, map_width: i32) -> f32 {
    hex_radius * SQRT_3 * (map_width as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_around() {
        let coord1 = HexCoord::from_col_row(0, 0);
        let coord2 = HexCoord::from_col_row(MAP_WIDTH, 0);
        assert_eq!(coord1, coord2);

        let coord3 = HexCoord::from_col_row(-1, 0);
        let coord4 = HexCoord::from_col_row(MAP_WIDTH - 1, 0);
        assert_eq!(coord3, coord4);
    }

    #[test]
    fn test_wrap_distance() {
        let left_edge = HexCoord::from_col_row(0, 0);
        let right_edge = HexCoord::from_col_row(MAP_WIDTH - 1, 0);
        // 東西の端同士の距離は横ループにより 1 になる
        assert_eq!(left_edge.distance(right_edge), 1);
    }

    #[test]
    fn test_neighbors_wrap() {
        let left_edge = HexCoord::from_col_row(0, 0);
        let neighbors = left_edge.neighbors();
        let right_edge = HexCoord::from_col_row(MAP_WIDTH - 1, 0);
        assert!(neighbors.contains(&right_edge));
    }
}
