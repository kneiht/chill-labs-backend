use chillteacher::settings::Settings;
use chillteacher::utils::email::EmailService;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    // Set up logging
    tracing_subscriber::fmt::init();

    // Load settings from environment variables
    let settings = Settings::new("CHILLTEACHER")?;

    // Print settings
    println!("Settings: {:#?}", &settings);

    // Create the email service from settings
    let email_config = chillteacher::utils::email::EmailConfig {
        smtp_host: settings.email.smtp_host.unwrap(),
        smtp_port: 2525,
        smtp_username: settings.email.smtp_username.unwrap(),
        smtp_password: settings.email.smtp_password.unwrap(),
        from_email: settings.email.from_email.unwrap(),
        from_name: settings.email.from_name.unwrap(),
        verification_url_base: settings.email.verification_url_base.unwrap(),
    };

    let email_service = EmailService::new(email_config);

    tracing::info!(
        "Creating email service with config: {:#?}",
        email_service.config
    );

    // Send a test verification email
    println!("Sending test email...");
    match email_service
        .send_verification_email(
            "test@example.com", // Replace with your test email
            "Test User",
            "test-verification-token-123",
        )
        .await
    {
        Ok(_) => println!("Email sent successfully! Check your Mailtrap inbox."),
        Err(e) => {
            println!("Failed to send email: {}", e);
            let mut source = e.source();
            let mut i = 0;
            while let Some(err) = source {
                println!("  Cause {}: {}", i, err);
                source = err.source();
                i += 1;
            }
        }
    }

    Ok(())
}
