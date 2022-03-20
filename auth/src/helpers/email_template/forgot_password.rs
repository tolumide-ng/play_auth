pub fn forgot_template (url: String) -> String {
    let message = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta http-equiv="X-UA-Compatible" content="IE=edge">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Document</title>
        </head>
        <body>
            <h4>Welcome to Cereal</h4>
            <div class="">
                <p>Please click on the button below to reset password, this link expires in 20 minutes, please ignore the message if you did not request for a password change</p>
                <p><a href="{url}" target="_blank" rel="noopener noreferrer">Activate your account</a></p>
            </div>
        </body>
        </html>
        "#
    );

    return message
}