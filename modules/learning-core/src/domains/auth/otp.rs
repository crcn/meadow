use tracing::info;
use twilio::{TwilioOptions, TwilioService};

use crate::error::{Error, Result};

const TEST_PHONE: &str = "+1234567890";
const TEST_CODE: &str = "123456";

pub struct OtpService {
    twilio: TwilioService,
    test_enabled: bool,
}

impl OtpService {
    pub fn new(account_sid: String, auth_token: String, service_sid: String, test_enabled: bool) -> Self {
        let twilio = TwilioService::new(TwilioOptions {
            account_sid,
            auth_token,
            service_id: service_sid,
        });
        Self { twilio, test_enabled }
    }

    pub async fn send(&self, phone: &str) -> Result<()> {
        if self.test_enabled && phone == TEST_PHONE {
            info!("Test phone number — skipping Twilio OTP send");
            return Ok(());
        }

        self.twilio
            .send_otp(phone)
            .await
            .map(|_| ())
            .map_err(|e| Error::Auth(e.to_string()))
    }

    pub async fn verify(&self, phone: &str, code: &str) -> Result<()> {
        if self.test_enabled && phone == TEST_PHONE {
            info!("Test phone number — skipping Twilio OTP verify (any code accepted)");
            return Ok(());
        }

        self.twilio
            .verify_otp(phone, code)
            .await
            .map_err(|e| Error::Auth(e.to_string()))
    }
}
