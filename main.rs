//  "  TEST  CODE FOR SENDING E-MAIL  FOR  IN HTML FORMAT "

use lettre::message::{Mailbox, Message, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, Tokio1Executor, transport::smtp::response::Response};
use anyhow::Result;
use lettre::AsyncTransport;


#[tokio::main]
async fn main() -> Result<()> {
    // Replace these with real values before running
    let sender_addr = "manishsahacnb@gmail.com";
    let sender_name = "Manish Sah";
    let receiver_addr = "sonusahcnd@gmail.com";
    let receiver_name = "SONU";
    let app_password = "gatv gche bfhr iafu"; // Gmail App Password (if using Gmail)

    // Load or build your HTML content. You can also read from a file if you prefer.
    let html_body = r#"
<!doctype html>
<html>
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width,initial-scale=1" />
    <title>Test HTML Email</title>
    <style>
      body { font-family: system-ui, -apple-system, "Segoe UI", Roboto, "Helvetica Neue", Arial; }
      .card { border-radius: 8px; padding: 16px; box-shadow: 0 2px 6px rgba(0,0,0,0.08); }
      h1 { color: #111827; }
      p { color: #374151; }
    </style>
  </head>
  <body>
    <div class="card">
      <h1>Hello from Rust + lettre (async)!</h1>
      <p>This is an <strong>HTML</strong> email sent asynchronously using <code>lettre</code> with the <code>tokio1</code> feature.</p>
      <p>— Cheers</p>
    </div>
  </body>
</html>
"#;

    // Build the message
    let email = Message::builder()
        .from(Mailbox::new(Some(sender_name.into()), sender_addr.parse()?))
        .to(Mailbox::new(Some(receiver_name.into()), receiver_addr.parse()?))
        .subject("Async HTML email from Rust")
        .singlepart(SinglePart::html(html_body.to_string()))?;

    // Credentials
    let creds = Credentials::new(sender_addr.to_string(), app_password.to_string());

    // Create async SMTP transport using Tokio executor
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")?
            .credentials(creds)
            .build();

    // Send the email asynchronously
    match mailer.send(email).await {
        Ok(response) => print_success(response),
        Err(e) => eprintln!("Failed to send email: {:#?}", e),
    }

    Ok(())

    
}

fn print_success(resp: Response) {
    // Response gives SMTP server response info; printing for debugging
    println!("Email queued/sent. SMTP response: {} {}", resp.code(), resp.message().collect::<Vec<_>>().join(" "));
}
