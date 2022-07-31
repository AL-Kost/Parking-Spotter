use lettre::{
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, Message, Tokio02Connector,
};
use lettre_email::EmailBuilder;
use rodio::{Decoder, OutputStream, Sink};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

pub struct AlertSystem {
    config: crate::utils::Config,
    smtp_transport: AsyncSmtpTransport<Tokio02Connector>
}

impl AlertSystem {
    pub fn new(config: &crate::utils::Config) -> Result<Self, Box<dyn Error>> {
        // Set up the email transport
        let smtp_credentials = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());
        let smtp_connector = Tokio02Connector::new();
        let smtp_transport = AsyncSmtpTransport::<Tokio02Connector>::relay(&config.smtp_server)?
            .credentials(smtp_credentials)
            .connector(smtp_connector)
            .build();

        Ok(
            smtp_transport
          )
    }

    pub async fn send_alert(&self) -> Result<(), Box<dyn Error>> {
        // Send an email notification
        let email = EmailBuilder::new()
            .to(self.config.email_to.clone())
            .from(self.config.email_from.clone())
            .subject(self.config.email_subject.clone())
            .text(self.config.email_body.clone())
            .build()?;

        let email_message = Message::from(email.into());
        self.smtp_transport.send(email_message).await?;

        // Play the sound notification
        self.play_sound_notification()?;

        // Sleep for a specified interval before allowing another alert
        let interval = Duration::from_millis(self.config.detection_interval_ms);
        sleep(interval).await;

        Ok(())
    }

    fn play_sound_notification(&self) -> Result<(), Box<dyn Error>> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = rodio::Sink::try_new(&stream_handle)?;

        let file = File::open(&self.config.sound_file_path)?;
        let source = Decoder::new(BufReader::new(file))?;
        sink.append(source);

        sink.sleep_until_end();

        Ok(())
    }
}
