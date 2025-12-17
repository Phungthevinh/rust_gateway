//định nghĩa struct và enum trong ratelimit

pub struct RateLimitEntry {
    pub key: String,         // đầu vào của số điện thoại, hoặc các yêu cầu khác, có thể là IP
    pub rule: RateLimitRule, // quy tắc quy định cho RateLimitRule
}

pub enum RateLimitRule {
    Unlimited,
    // dùng để chia thời gian cho mỗi requert gửi đi
    // chia ra để xử lý
    //ví dụ mỗi window tối đa gửi 100 requert trong khoảng thời gian quy định
    FixedWindow {
        max_requests: u32,
        window_seconds: u64,
    },
    //không chia thời gian cố định
    //vi dụ trong 60s chỉ nhận được 100 requert, nếu quá requert quy định sẽ ngưng ko nhận nữa
    SlidingWindow {
        max_requests: u32,
        window_seconds: u64,
    },
}


