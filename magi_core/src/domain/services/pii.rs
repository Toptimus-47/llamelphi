use regex::Regex;
use std::sync::OnceLock;

static EMAIL_RE: OnceLock<Regex> = OnceLock::new();
static PHONE_RE: OnceLock<Regex> = OnceLock::new();
static RRN_RE: OnceLock<Regex> = OnceLock::new();

pub struct PiiService;

impl PiiService {
    pub fn mask_pii(text: &str) -> String {
        let email_re = EMAIL_RE.get_or_init(|| Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap());
        let phone_re = PHONE_RE.get_or_init(|| Regex::new(r"010-\d{3,4}-\d{4}").unwrap());
        let rrn_re = RRN_RE.get_or_init(|| Regex::new(r"\d{6}-[1-4]\d{6}").unwrap());

        let mut result = text.to_string();

        result = email_re.replace_all(&result, "[EMAIL_HIDDEN]").to_string();
        result = phone_re.replace_all(&result, "[PHONE_HIDDEN]").to_string();
        result = rrn_re.replace_all(&result, "[ID_HIDDEN]").to_string();

        result
    }

    /// 정교화된 마스킹 로직 (향후 확장 가능)
    pub fn mask_all_sensitive(text: &str) -> String {
        Self::mask_pii(text)
    }
}
