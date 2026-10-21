//! Complete prose for the thirteen-chapter story, used only by the seed command.
pub(super) const SLUG: &str = "ben-cu-dong-ngan";
pub(super) const TITLE: &str = "Bến Cũ Đông Ngàn";
pub(super) const CHAPTER_ONE: &str = CONTENTS[0];
pub(super) const TITLES: &[&str; 13] = &[
    "Chén trà còn ấm",
    "Ngọn đèn bên kia bến",
    "Người vá những mùa mưa",
    "Lá thư không ghi ngày",
    "Đường lên đồi hoa trắng",
    "Một mái hiên cho gió",
    "Tiếng đàn qua xóm vắng",
    "Mùa nước chậm",
    "Chiếc thuyền chở hoa",
    "Điều cha chưa kịp nói",
    "Đêm thả đèn trên sông",
    "Người ở lại cùng mùa xuân",
    "Bến cũ, trời Đông Ngàn",
];

pub(super) const CONTENTS: &[&str; 13] = &[
    include_str!("01_chen_tra_con_am.txt"),
    include_str!("02_ngon_den_ben_kia_ben.txt"),
    include_str!("03_nguoi_va_nhung_mua_mua.txt"),
    include_str!("04_la_thu_khong_ghi_ngay.txt"),
    include_str!("05_duong_len_doi_hoa_trang.txt"),
    include_str!("06_mot_mai_hien_cho_gio.txt"),
    include_str!("07_tieng_dan_qua_xom_vang.txt"),
    include_str!("08_mua_nuoc_cham.txt"),
    include_str!("09_chiec_thuyen_cho_hoa.txt"),
    include_str!("10_dieu_cha_chua_kip_noi.txt"),
    include_str!("11_dem_tha_den_tren_song.txt"),
    include_str!("12_nguoi_o_lai_cung_mua_xuan.txt"),
    include_str!("13_ben_cu_troi_dong_ngan.txt"),
];
