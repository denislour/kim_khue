//! Story data shared by the library and its reading preview.
//! These illustrative stories can later be replaced by the author's own work.

pub struct Story {
    pub id: &'static str,
    pub title: &'static str,
    pub genre: &'static str,
    pub genre_key: &'static str,
    pub cover: &'static str,
    pub summary: &'static str,
    pub chapter: &'static str,
    pub excerpt: &'static [&'static str],
}

pub static SAMPLE_STORIES: &[Story] = &[
    Story {
        id: "river",
        title: "Bến cũ trăng ngàn",
        genre: "Kiếm hiệp",
        genre_key: "wuxia",
        cover: "mountain",
        summary: "Một người trở lại bến sông sau mười năm. Chuyến đò năm ấy đã đi, chỉ lời hẹn xưa còn ở lại.",
        chapter: "Khởi đầu · Người về bên sông",
        excerpt: &[
            "Mười năm rồi, bến sông vẫn có một cây liễu nghiêng về phía nước. Chỉ khác là người chèo đò năm xưa đã bạc tóc, còn thanh kiếm bên hông khách lạ thì không còn vỏ.",
            "Ông lão ngẩng đầu nhìn người vừa đến. Không hỏi từ đâu, cũng không hỏi về đâu. Ông chỉ đẩy chén trà còn ấm về phía khoảng trống trước mặt.",
            "“Qua sông chứ?”",
            "Người khách đặt tay lên chuôi kiếm, rồi buông xuống. Phía bên kia, một ngọn đèn vừa được thắp trong căn nhà cũ. Lần đầu tiên suốt mười năm, chàng không biết mình nên trả lời thế nào.",
        ],
    },
    Story {
        id: "mai",
        title: "Thư gửi mùa hoa cũ",
        genre: "Cổ trang",
        genre_key: "romance",
        cover: "mai",
        summary: "Trong một phong thư chưa gửi, nàng cất cả mùa xuân cuối cùng của kinh thành.",
        chapter: "Khởi đầu · Một cành mai muộn",
        excerpt: &[
            "Lá thư nằm dưới đáy hộp gỗ, bên cạnh một cành mai đã khô. Nàng không nhớ mình đã đặt nó ở đó từ bao giờ. Có lẽ là một buổi chiều mưa, khi tiếng vó ngựa cuối cùng vừa khuất sau cổng thành.",
            "Ngoài hiên, người hầu nhỏ đang quét những cánh hoa rơi. Mỗi nhát chổi rất khẽ, như sợ đánh thức một điều gì đã ngủ yên trong sân.",
            "Nàng mở cửa sổ. Hương trà mới thoảng lên từ gian phòng bên, làm nàng bỗng nhớ một lời dặn cũ: khi mùa xuân trở lại, hãy tự pha cho mình một ấm trà ngon.",
            "Lần này, nàng lấy giấy mới. Không ghi tên người nhận. Chỉ viết rằng hoa trong vườn năm nay đã nở.",
        ],
    },
    Story {
        id: "moon",
        title: "Người giữ đèn sương",
        genre: "Huyền huyễn",
        genre_key: "fantasy",
        cover: "moon",
        summary: "Giữa núi mù sương, có một ngọn đèn chỉ sáng lên khi ai đó quên mất đường về.",
        chapter: "Khởi đầu · Đèn sáng trong sương",
        excerpt: &[
            "Không ai biết ngôi nhà xuất hiện trên núi từ khi nào. Người hái thuốc nói nó chỉ có vào những ngày nhiều sương. Người đốn củi thì bảo mình từng nghe tiếng ấm nước reo ở đó giữa đêm mùa đông.",
            "Trong nhà, một cô gái đang châm thêm dầu vào ngọn đèn. Ánh sáng xanh nhạt chạm vào từng nếp gấp của tấm bản đồ trải trên bàn. Những con đường trên giấy chuyển động rất chậm.",
            "Có tiếng gõ cửa. Ba tiếng, ngập ngừng.",
            "Cô gái ngước mắt. Trên bản đồ, một con đường vừa hiện ra. Nó nhỏ như một sợi chỉ, dẫn từ chân núi đến một nơi cô chưa từng đặt tên.",
        ],
    },
];
