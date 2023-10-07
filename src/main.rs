use colored::Colorize;
use inquire::{
    ui::{Attributes, RenderConfig, StyleSheet, Styled},
    Confirm, Select, Text,
};
use std::fs::OpenOptions;
use std::io::{Read, Write};

use aws_apis::{
    load_credential_from_env, CredentInitialize, MemDbOps, RdsOps, S3Ops, SesOps, SimpleMail,
    Simple_, TemplateMail, Template_,
};
use dotenv::dotenv;
use reqwest::get;
use std::env::var;
#[tokio::main]
async fn main() {
    inquire::set_global_render_config(global_render_config());
    let operations: Vec<&str> = vec![
        "Verify the Credential\n",
        "Print Credentials Information\n",
        "AWS Simple Email Service(SES) Operations\n",
        "S3 Bucket Operations\n",
        "Relational Database Service(RDS) Operations\n",
        "MemoryDb Operations\n",
        "Quit the application\n",
    ];
    //Intial dummy credentials
    let mut credential = CredentInitialize::default();
    let mut s3_ops: S3Ops = S3Ops::build(credential.build());
    let mut ses_ops: SesOps = SesOps::build(credential.build());
    let mut rds_ops: RdsOps = RdsOps::build(credential.build());
    let mut memdb_ops: MemDbOps = MemDbOps::build(credential.build());
    'main: loop {
        let choice = Select::new(
            "Select the option to execute the operation\n",
            operations.clone(),
        )
        .with_help_message(
            "Don't enclose data in quotation marks or add spaces around it in any operations",
        )
        .with_page_size(8)
        .prompt()
        .unwrap();

        match choice {
            "Verify the Credential\n" => {
                let choices = Confirm::new("Load the credentials from the configuration file or from environment variables\n")
                          .with_placeholder("Use 'Yes' to load from the environment and 'No' to load from environment variables\n")
                          .with_help_message("Without proper credentials, no operations can be executed successfully")
                          .prompt()
                          .unwrap();

                match choices {
                    true => {
                        let (credentials, region) = load_credential_from_env().await;
                        credential.update(
                            credentials.access_key_id(),
                            credentials.secret_access_key(),
                            region.as_deref(),
                        );
                        let config = credential.build();
                        ses_ops = SesOps::build(config.clone());
                        s3_ops = S3Ops::build(config.clone());
                        rds_ops = RdsOps::build(config.clone());
                        memdb_ops = MemDbOps::build(config.clone());
                        println!("{}\n","Please verify the credentials by printing the credential information before proceeding with any operations".blue().bold());
                    }
                    false => {
                        dotenv().ok();
                        let access_key = var("AWS_ACCESS_KEY_ID")
                        .expect("Ensure that the 'AWS_ACCESS_KEY_ID' environment variable is set, and its value is provided by AWS\n");
                        let secret_key = var("AWS_SECRET_ACCESS_KEY")
                        .expect("Ensure that the 'AWS_SECRET_ACCESS_KEY' environment variable is set, and its value is provided by AWS\n");
                        let region = var("AWS_DEFAULT_REGION")
                        .expect("Ensure that the 'AWS_DEFAULT_REGION' environment variable is set, and its value is provided by AWS\n");
                        credential.update(&access_key, &secret_key, Some(&region));
                        let config = credential.build();
                        ses_ops = SesOps::build(config.clone());
                        s3_ops = S3Ops::build(config.clone());
                        rds_ops = RdsOps::build(config.clone());
                        memdb_ops = MemDbOps::build(config.clone());
                        println!("{}\n","Please verify the credentials by printing the credential information before proceeding with any operations".red().bold());
                    }
                }
            }
            "Print Credentials Information\n" => {
                let confirm = Confirm::new(
                    "Are you sure you want to print credential information?\n",
                )
                .with_formatter(&|str| format!(".....{str}.....\n"))
                .with_placeholder(
                    "Type 'Yes' to view the credentials, or 'No' to not view the credentials\n",
                )
                .with_help_message("This is solely for verification purposes")
                .with_default(false)
                .prompt()
                .unwrap();

                match confirm {
                    true => {
                        println!("Here is your credential informations");
                        println!("{:#?}\n", credential.get_credentials());
                    }
                    false => {
                        println!("{}\n", "Sure...".green().bold())
                    }
                }
            }
            "AWS Simple Email Service(SES) Operations\n" => {
                let ses_operations = vec![
                    "Create a Contact List Name\n",
                    "Add an email to the list\n",
                    "Send a Single Simple Email\n",
                    "Send a Bulk of Simple Emails\n",
                    "Default Values\n",
                    "Create Email Template\n",
                    "Get Email Template\n",
                    "Get Email Template Variables\n",
                    "Send a Single Templated Email\n",
                    "Send a Bulk of Templated Emails\n",
                    "Retrieve emails from the provided list\n",
                    "Create Email Identity\n",
                    "Email Verification\n",
                    "Get Email Identities\n",
                    "Update Email Template\n",
                    "Delete Template\n",
                    "Delete Contact List Name\n",
                    "Common Errors\n",
                    "Return to the Main Menu\n",
                ];
                loop {
                    let email_choice = Select::new(
                        "Select the option to execute the operation\n",
                        ses_operations.clone(),
                    )
                    .with_help_message("Do not enclose it with quotation marks or add spaces")
                    .with_vim_mode(true)
                    .with_page_size(10)
                    .prompt()
                    .unwrap();

                    match email_choice {
                        "Create Email Template\n" => {
                            let get_available_template_names = ses_ops.list_email_templates().await;
                            let placeholder_info = format!("Please note that these template names are already available for your use:\n{:#?}",get_available_template_names);
                            let template_name = Text::new(
                                "Please provide the new template name for this template\n",
                            )
                            .with_placeholder(&placeholder_info)
                            .with_formatter(&|input| {
                                format!("Received Template Name Is: {input}\n")
                            })
                            .prompt_skippable()
                            .unwrap()
                            .unwrap();
                            let subject_path =Text::new("Please provide the path to the subject data in HTML format to create Subject for Email Template\n")
                                .with_placeholder("The subject can contain template variables to personalize the email template's subject line\nDo not use apostrophes, spaces, or commas around template variables\n")
                                .with_help_message("An example subject template is available here https://tinyurl.com/4etkub75 ")
                                .with_formatter(&|input| format!("Received Subject Is: {input}\n"))
                                .prompt()
                                .unwrap();

                            let template_path = Text::new("Please provide the path for the template in HTML format to Create a HTML body for the Email Template\n")
                                      .with_formatter(&|input| format!("Received Template Path Is: {input}\n"))
                                      .with_placeholder("The HTML body can contain both template variables and HTML content\n")
                                      .with_help_message("Example template is available at this location: https://tinyurl.com/rmxwfc5v")
                                      .prompt()
                                      .unwrap();

                            let text_path =Text::new("Please provide the path to the text body for the email template\n")
                                .with_placeholder("This section is optional, but it's essential to include for recipients who do not support HTML\n")
                                .with_formatter(&|input| format!("Received Text Body Is: {input}\n"))
                                .with_help_message("Example text body data is available here https://tinyurl.com/ycy4sbmn")
                                .prompt_skippable()
                                .unwrap()
                                .unwrap();
                            match (
                                template_name.is_empty(),
                                subject_path.is_empty(),
                                template_path.is_empty(),
                            ) {
                                (false, false, false) => {
                                    let mut reading_template_data = OpenOptions::new()
                                        .read(true)
                                        .write(true)
                                        .open(&template_path)
                                        .expect(
                                            "Error opening the Template file path you specified\n",
                                        );
                                    let mut template_data = String::new();
                                    reading_template_data
                                        .read_to_string(&mut template_data)
                                        .expect("Error while reading data\n");
                                    let mut reading_subject_data = OpenOptions::new()
                                        .read(true)
                                        .write(true)
                                        .open(&subject_path)
                                        .expect(
                                            "Error opening the Subject file path you specified\n",
                                        );
                                    let mut subject_data = String::new();
                                    reading_subject_data
                                        .read_to_string(&mut subject_data)
                                        .expect("Error while reading data\n");

                                    match text_path.is_empty() {
                                        false => {
                                            let mut reading_text_data = OpenOptions::new()
                                                                 .read(true)
                                                                 .write(true)
                                                                 .open(&text_path)
                                                                 .expect("Error opening the Text Body file path you specified\n");
                                            let mut text_data = String::new();
                                            reading_text_data
                                                .read_to_string(&mut text_data)
                                                .expect(
                                                    "Error opening the file path you specified\n",
                                                );

                                            ses_ops
                                                .create_email_template(
                                                    &template_name,
                                                    &subject_data,
                                                    &template_data,
                                                    Some(text_data),
                                                )
                                                .await;
                                        }
                                        true => {
                                            ses_ops
                                                .create_email_template(
                                                    &template_name,
                                                    &subject_data,
                                                    &template_data,
                                                    None,
                                                )
                                                .await;
                                        }
                                    }
                                }
                                _ => {
                                    println!("{}\n", "Fields should not be left empty".red().bold())
                                }
                            }
                        }
                        "Update Email Template\n" => {
                            let get_available_template_names = ses_ops.list_email_templates().await;
                            let placeholder_info = format!(
                                "Available Template Names in Your Credentials\n{:#?}",
                                get_available_template_names
                            );
                            let template_name = Text::new(
                                "Please provide the template name to update the associated template\n",)
                            .with_placeholder(&placeholder_info)
                            .with_formatter(&|input| format!("Received Template Name Is: {input}\n"))
                            .prompt()
                            .unwrap();
                            match template_name.is_empty() {
                                false => {
                                    let (current_subject, current_template_html, current_text) =
                                        ses_ops
                                            .get_template_subject_html_and_text(
                                                &template_name,
                                                false,
                                            )
                                            .await;
                                    let current_subject = format!(
                                        "Your current email template subject is:\n {}",
                                        current_subject
                                    );
                                    let subject_path =Text::new("Please provide the path to the subject data in JSON format to update\n")
                                .with_placeholder(&current_subject)
                                .with_formatter(&|input| format!("Received Subject Is: {input}\n"))
                                .prompt()
                                .unwrap();
                                    let current_template_variables = ses_ops
                                        .get_template_variables_of_subject_and_html_body(
                                            &current_subject,
                                            &current_template_html,
                                        );
                                    let current_template_variables = format!("These are the current template variables in the template named '{}'\n{}",template_name,current_template_variables.1.join("\n"));
                                    let template_path = Text::new("Please provide the path for the template in JSON format to update it with the old one\n")
                                      .with_formatter(&|input| format!("Received Template Path Is: {input}\n"))
                                      .with_placeholder(&current_template_variables)
                                      .with_help_message("Example template is available at this location: https://tinyurl.com/4na92rph")
                                      .prompt()
                                      .unwrap();
                                    let current_text = format!(
                                        "Your current email template text is:\n{}\n",
                                        current_text
                                    );
                                    let text_path =Text::new("Please provide the path to the text body for the email template\n")
                                .with_placeholder(&current_text)
                                .with_help_message("This section is optional, but it's essential to include for recipients who do not support HTML")
                                .with_formatter(&|input| format!("Received Text Body Is: {input}\n"))
                                .prompt_skippable()
                                .unwrap()
                                .unwrap();
                                    let mut reading_template_data = OpenOptions::new()
                                        .read(true)
                                        .write(true)
                                        .open(&template_path)
                                        .expect(
                                            "Error opening the Template file path you specified\n",
                                        );
                                    let mut template_data = String::new();
                                    reading_template_data
                                        .read_to_string(&mut template_data)
                                        .expect("Error while reading template data\n");
                                    let mut reading_subject_data = OpenOptions::new()
                                        .read(true)
                                        .write(true)
                                        .open(&subject_path)
                                        .expect(
                                            "Error opening the Subject file path you specified",
                                        );
                                    let mut subject_data = String::new();
                                    reading_subject_data
                                        .read_to_string(&mut subject_data)
                                        .expect("Error while reading subject data\n");

                                    match text_path.is_empty() {
                                        false => {
                                            let mut read_text_data = OpenOptions::new()
                                    .read(true)
                                    .write(true)
                                    .open(&text_path)
                                    .expect("Error opening the Text Body file path you specified\n");
                                            let mut text = String::new();
                                            read_text_data
                                                .read_to_string(&mut text)
                                                .expect("Error While Reading to String ");
                                            ses_ops
                                                .update_template(
                                                    &template_name,
                                                    &subject_data,
                                                    &template_data,
                                                    Some(text),
                                                )
                                                .await;
                                        }
                                        true => {
                                            ses_ops
                                                .update_template(
                                                    &template_name,
                                                    &subject_data,
                                                    &template_data,
                                                    None,
                                                )
                                                .await;
                                        }
                                    }
                                }
                                true => {
                                    println!("{}\n", "Template Name can't be empty".red().bold())
                                }
                            }
                        }
                        "Get Email Template\n" => {
                            let get_available_template_names = ses_ops.list_email_templates().await;
                            let placeholder_info = format!(
                                "Available Template Names in Your Credentials\n{:#?}",
                                get_available_template_names
                            );
                            let template_name = Text::new("Please provide the template name\n")
                                .with_placeholder(&placeholder_info)
                                .with_formatter(&|input| {
                                    format!("Received Template Name Is: {input}\n")
                                })
                                .prompt()
                                .unwrap();
                            match template_name.is_empty() {
                                false => {
                                    ses_ops
                                        .get_template_subject_html_and_text(&template_name, true)
                                        .await;
                                }
                                true => {
                                    println!("{}\n", "Template Name can't be empty".red().bold())
                                }
                            }
                        }
                        "Get Email Template Variables\n" => {
                            let get_available_template_names = ses_ops.list_email_templates().await;
                            let placeholder_info = format!(
                                "Available Template Names in Your Credentials\n{:#?}",
                                get_available_template_names
                            );
                            let template_name = Text::new(
                                "Please provide the new template name for this template\n",
                            )
                            .with_placeholder(&placeholder_info)
                            .with_formatter(&|input| {
                                format!("Received Template Name Is: {input}\n")
                            })
                            .prompt_skippable()
                            .unwrap()
                            .unwrap();
                            match template_name.is_empty() {
                                false => {
                                    let (subject_data, template_data, _) = ses_ops
                                        .get_template_subject_html_and_text(&template_name, false)
                                        .await;
                                    let (subject_variables, html_variables) = ses_ops
                                        .get_template_variables_of_subject_and_html_body(
                                            &subject_data,
                                            &template_data,
                                        );
                                    println!(
                                        "{}\n",
                                        "Subject Template Variables if any".yellow().bold()
                                    );
                                    subject_variables.into_iter().for_each(|variable| {
                                        println!("    {}", variable.green().bold());
                                    });
                                    println!("");
                                    println!(
                                        "{}\n",
                                        "HTML Template Variables if any".yellow().bold()
                                    );
                                    html_variables.into_iter().for_each(|variable| {
                                        println!("    {}", variable.green().bold());
                                    });
                                    println!("");
                                }
                                true => {
                                    println!("{}\n", "Template Name can't be empty".red().bold())
                                }
                            }
                        }
                        "Delete Template\n" => {
                            let get_available_template_names = ses_ops.list_email_templates().await;
                            let placeholder_info = format!(
                                "Available Templates \n{:#?}",
                                get_available_template_names
                            );
                            let template_name =
                                Text::new("Please provide the template name for deletion\n")
                                    .with_placeholder(&placeholder_info)
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt_skippable()
                                    .unwrap()
                                    .unwrap();
                            match template_name.is_empty() {
                                false => {
                                    ses_ops.delete_template(&template_name).await;
                                }
                                true => {
                                    println!("{}\n", "Template Name can't be empty".red().bold())
                                }
                            }
                        }

                        "Create a Contact List Name\n" => {
                            let lst_name = Text::new(
                                "Enter the list name to add to the AWS Simple Email Service\n",
                            )
                            .with_placeholder("The name should be unique\n")
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .with_help_message("This is where the emails are stored")
                            .prompt()
                            .unwrap();
                            let description = Text::new("Small Description about the list name\n")
                    .with_placeholder("Eg: A list named 'Zone Email Contacts' is used to add the emails\nof people in a specific area but can be skipped\n")
                    .with_formatter(&|str| format!(".....{str}.....\n"))
                    .prompt_skippable()
                    .unwrap()
                    .unwrap();
                            match (lst_name.is_empty(), description.is_empty()) {
                                (false, false) => {
                                    ses_ops
                                        .create_contact_list_name(&lst_name, Some(description))
                                        .await;
                                }
                                (false, true) => {
                                    ses_ops.create_contact_list_name(&lst_name, None).await;
                                }
                                _ => println!(
                                    "{}\n",
                                    "Contact Name Can't be empty..try again".red().bold()
                                ),
                            }
                        }
                        "Delete Contact List Name\n" => {
                            let get_available_contact_lists = ses_ops.list_contact_lists().await;
                            let contact_list_names = format!(
                                "Available Contact List Names:\n{:#?}\n",
                                get_available_contact_lists
                            );
                            let lst_name = Text::new("Enter the Contact List name to delete from AWS Simple Email Service\n")
                    .with_placeholder(&contact_list_names)
                    .with_formatter(&|str| format!(".....{str}.....\n"))
                    .with_help_message("This is where the emails are stored")
                    .prompt()
                    .unwrap();
                            match lst_name.is_empty() {
                                false => {}
                                true => println!(
                                    "{}\n",
                                    "Contact List Name can't be empty".red().bold()
                                ),
                            }
                        }

                        "Add an email to the list\n" => {
                            let get_contact_list_name = ses_ops.get_list_name();
                            let get_contact_list_name =
                                format!("Default contact list name: {}\n", get_contact_list_name);
                            let email = Text::new("Enter the email\n")
                                .with_placeholder(
                                    "Emails should be without quotation marks around them\n",
                                )
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .prompt()
                                .unwrap();
                            let list_name =
                                Text::new("Enter the list name you want the email add in it\n")
                                    .with_placeholder(&get_contact_list_name)
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt_skippable()
                                    .unwrap()
                                    .unwrap();
                            let to_verified = Confirm::new("Would you like to send the verification email as well?\n")
                                           .with_formatter(&|str| format!(".....{str}.....\n"))
                                           .with_placeholder("Selecting 'Yes' means you want to receive a verification, while choosing 'No' means your email will be added to the list without verification\n")
                                           .prompt()
                                           .unwrap();

                            match (list_name.is_empty(), email.is_empty(), to_verified) {
                                (false, false, false) => {
                                    ses_ops
                                        .create_email_contact_without_verification(
                                            &email,
                                            Some(&list_name),
                                        )
                                        .await;
                                    println!("You must pass the email '{}' to the 'Create Email Identity' option before sending an email to this address\n",email.yellow().bold());
                                }
                                (false, false, true) => {
                                    ses_ops
                                        .create_email_contact_with_verification(
                                            &email,
                                            Some(&list_name),
                                        )
                                        .await;
                                }
                                (true, false, false) => {
                                    ses_ops
                                        .create_email_contact_without_verification(&email, None)
                                        .await;
                                    println!("You must pass the email '{}' to the 'Create Email Identity' option before sending an email to this address\n",email.yellow().bold());
                                }
                                (true, false, true) => {
                                    ses_ops
                                        .create_email_contact_with_verification(&email, None)
                                        .await;
                                }
                                _ => println!("{}\n", "No email is received".red().bold()),
                            }
                        }
                        "Create Email Identity\n" => {
                            let email = Text::new("Enter the email\n")
                                .with_placeholder(
                                    "Emails should be without quotation marks around them\n",
                                )
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .prompt()
                                .unwrap();
                            match email.is_empty() {
                                false => {
                                    ses_ops.create_email_identity(&email).await;
                                }
                                true => println!("{}\n", "Email Can't be empty"),
                            }
                        }

                        "Email Verification\n" => {
                            let email_to_verify =
                                Text::new("Enter the email to check the identity\n")
                                    .with_placeholder("Only verified email can receive email\n")
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt()
                                    .unwrap();
                            match email_to_verify.is_empty() {
                                false => {
                                    let available_email_identies =
                                        ses_ops.retrieve_emails_from_list_email_identities().await;
                                    if available_email_identies.contains(&email_to_verify) {
                                        match ses_ops.is_email_verfied(&email_to_verify).await {
                                            true => {
                                                let email_to_verify =
                                                    email_to_verify.green().bold();
                                                println!("The email address {email_to_verify} has been verified\n");
                                                println!(" You can use it to receive messages or as a 'from' address\n");
                                            }
                                            false => {
                                                let email_to_verify =
                                                    email_to_verify.green().bold();
                                                println!("The email address {email_to_verify} is not verified\n");
                                                println!("Therefore, you can't use it to send emails ('from' address) or receive messages\n");
                                            }
                                        }
                                    } else {
                                        println!(
                                            "No identity was found for the email '{}'",
                                            email_to_verify
                                        );
                                        println!("{}\n","Please execute the 'create email identity' option before verifying this email".yellow().bold());
                                    }
                                }
                                true => {
                                    println!("{}\n", "The email can't be empty".red().bold())
                                }
                            }
                        }

                        "Retrieve emails from the provided list\n" => {
                            let get_contact_list_name = ses_ops.get_list_name();
                            let get_contact_list_name =
                                format!("Default contact list name: {}\n", get_contact_list_name);
                            let list_name = Text::new("Please enter the name of the list for which you'd like to receive these emails in PDF and text formats\n")
                                       .with_placeholder(&get_contact_list_name)
                                       .with_formatter(&|str| format!(".....{str}....."))
                                       .prompt_skippable()
                                       .unwrap()
                                       .unwrap();
                            match list_name.is_empty() {
                                false => {
                                    ses_ops
                                        .writing_email_addresses_from_provided_list_as_text_pdf(
                                            Some(&list_name),
                                        )
                                        .await;
                                }
                                true => {
                                    ses_ops
                                        .writing_email_addresses_from_provided_list_as_text_pdf(
                                            None,
                                        )
                                        .await;
                                }
                            }
                        }
                        "Default Values\n" => {
                            let default_list_name = ses_ops.get_list_name().green().bold();
                            let default_template_name = ses_ops.get_template_name().green().bold();
                            let default_from_address = ses_ops.get_from_address().green().bold();
                            println!("Default Contact List Name: {default_list_name}\n");
                            println!("Default Template Name: {default_template_name}\n");
                            println!("Default from_address is: {default_from_address}\n");

                            println!("{}\n","Be sure to check the placeholder for default values, allowing you to skip using the default value".bright_blue());
                        }

                        "Send a Single Simple Email\n" => {
                            let email = Text::new("Enter the email..\n")
                                .with_formatter(&|str| format!(".....{str}....."))
                                .with_placeholder(
                                    "The provided email should be verified through the 'Create Email Identity' option",
                                )
                                .prompt()
                                .unwrap();
                            let email_contacts =
                                ses_ops.retrieve_emails_from_list_email_identities().await;
                            match email.is_empty() {
                                false => {
                                    if email_contacts.contains(&email) {
                                        let subject = Text::new("Enter the subject of Email\n")
                                        .with_placeholder(
                                            "Eg: For testing purposes, we have launched a new product",
                                        )
                                        .with_formatter(&|str| format!(".....{str}....."))
                                        .prompt()
                                        .unwrap();

                                        let defaul_from_address = ses_ops.get_from_address();

                                        let default_from_address =format!("Your 'from_address' needs to be verified, which is typically your email\nand the default 'from_address' is {}",defaul_from_address);

                                        let from_address = Text::new("Please enter the 'From' address, or press Enter to use the default 'From' address, if one is available in the placeholder\n")
                            .with_placeholder(&default_from_address)
                            .with_formatter(&|str| format!(".....{str}....."))
                            .prompt_skippable()
                            .unwrap()
                            .unwrap();
                                        let body_info = Confirm::new("You can either provide the email body from a local file path or any S3 object URLs can be passed, and they should be publicly accessible. Not all links provide the exact content we requested\n")
                        .with_formatter(&|str| format!(".....{str}....."))
                        .with_placeholder("Please respond with 'Yes' to provide a local file or 'No' to provide a S3 Object Url link\n")
                        .prompt()
                        .unwrap();
                                        let from_address = match from_address.is_empty() {
                                            true => None,
                                            false => Some(from_address.as_str()),
                                        };
                                        match (subject.is_empty(), body_info) {
                                            (false, true) => {
                                                let body_path = Text::new("Please provide the path to the body of a simple email content file\n")
                            .with_formatter(&|str| format!(".....{str}....."))
                            .with_placeholder("Any file extension is acceptable as long as it can be read and contains only text content or an HTML body, without any template variables\n")
                            .with_help_message("You can download a example simple email content here https://tinyurl.com/mr22bh4f")
                            .prompt()
                            .unwrap();
                                                let mut reading_simple_data = OpenOptions::new()
                            .read(true)
                            .write(true)
                            .open(&body_path)
                            .expect("Error opening the simple email file path you specified");
                                                let mut body_data = String::new();
                                                reading_simple_data
                                                    .read_to_string(&mut body_data)
                                                    .expect("Error while reading to string\n");
                                                let simple_email =
                                                    SimpleMail::builder(&body_data, &subject)
                                                        .build();

                                                ses_ops
                                .send_mono_email(&email, Simple_(simple_email),from_address)
                                .await
                                .send()
                                .await
                                .map(|_|{
                                    let colored_email = email.green().bold();
                                    println!("A simple email has been successfully sent to '{}'\n{}\n",colored_email,"Please check your inbox to view it".yellow().bold())
                                  })
                                .expect("Error while Sending Simple Email\n");
                                            }
                                            (false, false) => {
                                                let body_link = Text::new("Please provide the link to the body of a simple email content file\n")
                                                   .with_formatter(&|str| format!(".....{str}.....\n"))
                                                   .with_placeholder("Any file extension is acceptable as long as it can be read and contains only text content or an HTML body, without any template variables\n")
                                                   .with_help_message("Visit this link https://tinyurl.com/3bx4yz6v to obtain an S3 URL that contains the simple email content")
                                                   .prompt()
                                                 .unwrap();
                                                match get(&body_link).await{
                                         Ok(body) => {
                                let body_data = body.text().await.expect("Error while getting text data\n");
                                let x: &[_] = &['\n','\r',' ','\x1b','\u{20}','\u{7f}','\u{80}'];
                                let body_data = body_data.trim_matches(x);
                                let simple_email = SimpleMail::builder(
                                  body_data,
                                  &subject
                                  )
                                  .build();
                                
                                 ses_ops
                                .send_mono_email(&email, Simple_(simple_email),from_address)
                                .await
                                .send()
                                .await
                                .map(|_|{
                                    let colored_email = email.green().bold();
                                    println!("A simple email has been successfully sent to '{}'\n{}\n",colored_email,"Please check your inbox to view it".yellow().bold())
                                   })
                                .expect("Error While Sending Simple Email\n");

                            }
                            Err(_) => println!("{}\n","The provided link doesn't seem to be working. Could you please check the link and try again?".red().bold())
                        }
                                            }
                                            _ => println!("{}\n", "Subject can't be empty"),
                                        }
                                    } else {
                                        println!("The provided email '{}' has not been verified. Please execute the 'Create Email Identity' option to verify the email address, and then proceed with this one\n",email.yellow().bold());
                                    }
                                }
                                true => {
                                    println!("{}\n", "Email can't be empty".red().bold());
                                }
                            }
                        }
                        "Get Email Identities\n" => {
                            ses_ops.writing_email_identies_details_as_text_pdf().await;
                            println!("{}\n","This option only returns the emails that are created either via the 'Create Email Identity' option or\nby choosing 'yes' in the 'Add an Email to the list' option when asked to send a verification email".yellow().bold());
                        }
                        "Send a Single Templated Email\n" => {
                            let get_from_address = ses_ops.get_from_address();
                            let get_template_name = ses_ops.get_template_name();
                            let default_template_name =
                                format!("Default template name is: {}", get_template_name);
                            let default_from_address =
                                format!("Default from_address is: {}", get_from_address);
                            let email = Text::new("Enter the email you wish to send\n")
                                .with_placeholder("The email must be verified")
                                .with_formatter(&|str| format!(".....{str}....."))
                                .prompt()
                                .unwrap();
                            let email_contacts =
                                ses_ops.retrieve_emails_from_list_email_identities().await;

                            match email.is_empty() {
                                false => {
                                    if email_contacts.contains(&email) {
                                        let template_name = Text::new(
                                            "Please enter the template name you want to use for the email\n",)
                                        .with_placeholder(&default_template_name)
                                        .with_formatter(&|str| format!(".....{str}.....\n"))
                                        .with_help_message(
                                            "The template name must exist, and the variables should be specified as key-value pairs according to the template\n",
                                        )
                                        .prompt()
                                        .unwrap();
                                        let from_address = Text::new("Enter the from address\n")
                                            .with_placeholder(&default_from_address)
                                            .with_formatter(&|str| format!(".....{str}.....\n"))
                                            .prompt_skippable()
                                            .unwrap()
                                            .unwrap();
                                        let placeholder_info = format!(
                                        "The template variables should reflect the '{}' template",
                                        template_name
                                    );
                                        let template_path = Text::new(
                                "You can provide the path to the template data in JSON format\n",
                            )
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .with_placeholder(&placeholder_info)
                            .prompt()
                            .unwrap();

                                        match (
                                            template_name.is_empty(),
                                            from_address.is_empty(),
                                            template_path.is_empty(),
                                        ) {
                                            (false, false, false) => {
                                                let mut reading_template_data = OpenOptions::new()
                                                    .read(true)
                                                    .write(true)
                                                    .open(&template_path)
                                                    .expect(
                                                        "Error opening the file path you specified",
                                                    );
                                                let mut template_data = String::new();
                                                reading_template_data
                                                    .read_to_string(&mut template_data)
                                                    .expect("Error while reading data\n");

                                                let email_content = TemplateMail::builder(
                                                    &template_name,
                                                    &template_data,
                                                )
                                                .build();
                                                ses_ops
                                                    .send_mono_email(
                                                        &email,
                                                        Template_(email_content),
                                                        Some(&from_address),
                                                    )
                                                    .await
                                                    .send()
                                                    .await
                                                    .map(|_| {
                                                        let colored_email = email.green().bold();
                                                        println!(
                                                            "The template email is send to: {}\n",
                                                            colored_email
                                                        )
                                                    })
                                                    .expect("Error while sending template mail\n");
                                            }
                                            (false, true, false) => {
                                                if email_contacts.contains(&email) {
                                                    let mut reading_template_data = OpenOptions::new()
                                    .read(true)
                                    .write(true)
                                    .open(&template_path)
                                    .expect(
                                        "Error opening the Template file path you specified",
                                    );
                                                    let mut template_data = String::new();
                                                    reading_template_data
                                                        .read_to_string(&mut template_data)
                                                        .expect("Error while reading data\n");
                                                    let email_content = TemplateMail::builder(
                                                        &template_name,
                                                        &template_data,
                                                    )
                                                    .build();
                                                    ses_ops
                                                        .send_mono_email(
                                                            &email,
                                                            Template_(email_content),
                                                            None,
                                                        )
                                                        .await
                                                        .send()
                                                        .await
                                                        .map(|_| {
                                                            let colored_email =
                                                                email.green().bold();
                                                            println!(
                                                            "The template email is send to: {}\n",
                                                            colored_email
                                                        )
                                                        })
                                                        .expect(
                                                            "Error while sending template mail\n",
                                                        );
                                                } else {
                                                    println!("The provided email '{}' has not been verified. Please execute the 'Create Email Identity' option to verify the email address, and then proceed with this one\n",email.yellow().bold());
                                                }
                                            }
                                            (true, true, false) => {
                                                let mut reading_template_data = OpenOptions::new()
                                                    .read(true)
                                                    .write(true)
                                                    .open(&template_path)
                                                    .expect(
                                                        "Error opening the file path you specified",
                                                    );
                                                let mut template_data = String::new();
                                                reading_template_data
                                                    .read_to_string(&mut template_data)
                                                    .expect("Error while reading data\n");
                                                let email_content = TemplateMail::builder(
                                                    &get_template_name,
                                                    &template_data,
                                                )
                                                .build();
                                                ses_ops
                                                    .send_mono_email(
                                                        &email,
                                                        Template_(email_content),
                                                        None,
                                                    )
                                                    .await
                                                    .send()
                                                    .await
                                                    .map(|_| {
                                                        let colored_email = email.green().bold();
                                                        println!(
                                                            "The template email is send to: {}\n",
                                                            colored_email
                                                        )
                                                    })
                                                    .expect("Error while sending template mail\n");
                                            }
                                            (true, false, false) => {
                                                let mut reading_template_data = OpenOptions::new()
                                                    .read(true)
                                                    .write(true)
                                                    .open(&template_path)
                                                    .expect(
                                                        "Error opening the file path you specified",
                                                    );
                                                let mut template_data = String::new();
                                                reading_template_data
                                                    .read_to_string(&mut template_data)
                                                    .expect("Error while reading data\n");
                                                let email_content = TemplateMail::builder(
                                                    &get_template_name,
                                                    &template_data,
                                                )
                                                .build();
                                                ses_ops
                                                    .send_mono_email(
                                                        &email,
                                                        Template_(email_content),
                                                        Some(&from_address),
                                                    )
                                                    .await
                                                    .send()
                                                    .await
                                                    .map(|_| {
                                                        let colored_email = email.green().bold();
                                                        println!(
                                                            "The template email is send to: {}\n",
                                                            colored_email
                                                        )
                                                    })
                                                    .expect("Error while sending template mail\n");
                                            }
                                            _ => {
                                                println!("{}\n","Please ensure that the fields are not empty, and then try again.".red().bold());
                                            }
                                        }
                                    } else {
                                        println!("The provided email '{}' has not been verified. Please execute the 'Create Email Identity' option to verify the email address, and then proceed with this one\n",email.yellow().bold());
                                    }
                                }
                                true => println!("{}\n", "Email can't be empty".red().bold()),
                            }
                        }
                        "Send a Bulk of Simple Emails\n" => {
                            let get_from_address = ses_ops.get_from_address();
                            let get_list_name = ses_ops.get_list_name();

                            let default_from_address =
                                format!("Default from_address is: {}\n", get_from_address);
                            let default_list_name =
                                format!("Default list name is: {}\n", get_list_name);

                            let list_name = Text::new("Please provide the name of the Contact List where all your verified emails are stored\n")
                    .with_placeholder(&default_list_name)
                    .with_formatter(&|input| format!("The Simple Email Content will be sent to each email address in the: {input} Contact List\n"))
                    .prompt_skippable()
                    .unwrap()
                    .unwrap_or(ses_ops.get_list_name().into());
                            let body_info = Confirm::new("You can either provide the email body from a local file path or any S3 object URLs can be passed, and they should be publicly accessible. Not all links provide the exact content we requested\n")
                .with_formatter(&|str| format!(".....{str}....."))
                .with_placeholder("Please respond with 'Yes' to provide a local file or 'No' to provide a S3 Object Url link\n")
                .with_help_message("The body data is the same for all emails in the list")
                .prompt()
                .unwrap();
                            //println!("The emails in the provided list should be verified; otherwise, the operation may fail. You can choose to skip this step by providing empty input and then proceed with the 'Get Email Identities' option\n".yellow().bold());
                            let subject = Text::new("Please enter the subject content that all your subscribers should be aware of\n")
                    .with_placeholder("The subject is the same for all emails\n")
                    .with_formatter(&|str| format!(".....{str}.....\n"))
                    .prompt()
                    .unwrap();
                            let from_address = Text::new("Enter the from address\n")
                                .with_placeholder(&default_from_address)
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .prompt_skippable()
                                .unwrap()
                                .unwrap();

                            match (subject.is_empty(), body_info) {
                                (false, true) => {
                                    let body_path = Text::new("Please provide the path to the body of a simple email content file\n")
                        .with_formatter(&|str| format!(".....{str}....."))
                        .with_placeholder("Any file extension is acceptable as long as it can be read and contains only text content or an HTML body, without any template variables\n")
                        .with_help_message("You can download a example simple email content here https://tinyurl.com/mr22bh4f")
                        .prompt()
                        .unwrap();
                                    let body_data = std::fs::read_to_string(&body_path).expect(
                                        "Error Opening the simple email file path you specified\n",
                                    );
                                    let simple_data = SimpleMail::builder(&body_data, &subject);

                                    match (list_name.is_empty(), from_address.is_empty()) {
                                        (false, false) => {
                                            ses_ops
                                                .send_multi_email_with_simple(
                                                    simple_data,
                                                    Some(&from_address),
                                                    Some(&list_name),
                                                )
                                                .await;
                                        }
                                        (true, true) => {
                                            ses_ops
                                                .send_multi_email_with_simple(
                                                    simple_data,
                                                    None,
                                                    None,
                                                )
                                                .await;
                                        }
                                        (false, true) => {
                                            ses_ops
                                                .send_multi_email_with_simple(
                                                    simple_data,
                                                    None,
                                                    Some(&list_name),
                                                )
                                                .await;
                                        }
                                        (true, false) => {
                                            ses_ops
                                                .send_multi_email_with_simple(
                                                    simple_data,
                                                    Some(&from_address),
                                                    None,
                                                )
                                                .await;
                                        }
                                    }
                                }
                                (false, false) => {
                                    let body_link = Text::new("Please provide the link to the body of a simple email content file\n")
                        .with_formatter(&|str| format!(".....{str}....."))
                        .with_placeholder("Any file extension is acceptable as long as it can be read and contains only text content or an HTML body, without any template variables\n")
                        .with_help_message("Visit this link https://tinyurl.com/3bx4yz6v to obtain an S3 URL that contains the simple email content")
                        .prompt()
                        .unwrap();
                                    match get(&body_link).await{
                            Ok(body) => {
                                let body_data = body.text().await.expect("Error while getting text data\n");
                                let x: &[_] = &['\n','\r',' ','\x1b','\u{20}','\u{7f}','\u{80}'];
                                let body_data = body_data.trim_matches(x);
                                let simple_data = SimpleMail::builder(&body_data, &subject);
                                match (list_name.is_empty(),from_address.is_empty()){
                                    (false,false) => {
                                         ses_ops
                                        .send_multi_email_with_simple(simple_data, Some(&from_address), Some(&list_name))
                                        .await; 
                                    }
                                    (true,true) =>{
                                        ses_ops
                                        .send_multi_email_with_simple(simple_data, None, None)
                                        .await;
                                    }
                                    (false,true) =>{
                                        ses_ops
                                        .send_multi_email_with_simple(simple_data, None, Some(&list_name))
                                        .await;
                                    }
                                    (true,false) =>{
                                       ses_ops
                                        .send_multi_email_with_simple(simple_data, Some(&from_address), None)
                                        .await;
                                    }
                                  }

                            }
                            Err(_) => println!("{}\n","The provided link doesn't seem to be working. Could you please check the link and try again?".red().bold())
                        }
                                }
                                _ => {
                                    println!(
                                        "{}\n",
                                        "Email,subject or body can't be empty".red().bold()
                                    );
                                }
                            }
                        }

                        "Send a Bulk of Templated Emails\n" => {
                            let get_from_address = ses_ops.get_from_address();
                            let get_template_name = ses_ops.get_template_name();
                            let get_list_name = ses_ops.get_list_name();

                            use std::env::var;
                            match (var("TEMPLATE_NAME"), var("FROM_ADDRESS"), var("LIST_NAME")) {
                                (Ok(_), Ok(_), Ok(_)) => {
                                    println!(
                                        "Template Name: {}\nFrom Address: {}\nList Name: {}\n",
                                        get_template_name.green().bold(),
                                        get_list_name.green().bold(),
                                        get_from_address.green().bold()
                                    );
                                    ses_ops.send_bulk_templated_emails().await;
                                }
                                _ => {
                                    println!(
                                        "{}\n",
                                        "This information is for Debugging Purposes"
                                            .yellow()
                                            .bold()
                                    );
                                    println!(
                                        "Template Name: {}\nFrom Address: {}\nList Name: {}\n",
                                        get_template_name.green().bold(),
                                        get_list_name.green().bold(),
                                        get_from_address.green().bold()
                                    );
                                }
                            }
                        }
                        "Common Errors\n" => {
                            let possible_errors = include_str!("./possible_errors.txt")
                                .yellow()
                                .italic()
                                .bold();
                            println!("{}\n", possible_errors);
                        }
                        "Return to the Main Menu\n" => continue 'main,

                        _ => {}
                    }
                }
            }

            "S3 Bucket Operations\n" => {
                let s3_operations = vec![
                    "Create Bucket\n",
                    "Default Region Name\n",
                    "Put object in a Bucket\n",
                    "Modifying Object Visibility\n",
                    "List objects from a Bucket\n",
                    "Download object from bucket\n",
                    "Retrieve a presigned URL for an object\n",
                    "Get Bucket Lists\n",
                    "Delete object from a bucket\n",
                    "Delete Bucket\n",
                    "Return to the Main Menu\n",
                ];

                's3_ops: loop {
                    let s3_choices = Select::new(
                        "Select the option to execute the operation\n",
                        s3_operations.clone(),
                    )
                    .with_page_size(10)
                    .prompt()
                    .unwrap();
                    match s3_choices {
                        "Create Bucket\n" => {
                            let get_bucket_lists = s3_ops.get_buckets().await;
                            let existing_buckets = format!(
                                "These buckets are already in your account: {:#?}",
                                get_bucket_lists
                            );
                            let bucket_name = Text::new("Please input the name of the bucket\n")
                                .with_placeholder(
                                    &existing_buckets
                                )
                                .with_help_message("The name must begin with a lowercase letter and should be unique\nAn AWS bucket is a type of object storage designed for storing objects")
                                .with_formatter(&|str| format!("The Bucket name is: {str} and assumes that region name is provided"))
                                .prompt()
                                .unwrap();
                            match bucket_name.is_empty() {
                                false => {
                                    s3_ops.create_bucket(&bucket_name).await;
                                }
                                true => println!(
                                    "{}\n",
                                    "The bucket name can't be empty; please try again"
                                        .red()
                                        .bold()
                                ),
                            }
                        }
                        "Default Region Name\n" => {
                            let default_region_name = var("REGION").unwrap_or("The region value is read from the .env file in the current directory if it is not provided in the credential file".into());
                            println!("Default Region Name: {default_region_name}\n");
                        }
                        "Get Bucket Lists\n" => {
                            let bucket_lists = s3_ops.get_buckets().await;
                            for bucket in bucket_lists {
                                println!("    {}\n", bucket.green().bold());
                            }
                        }

                        "List objects from a Bucket\n" => {
                            let get_bucket_name = s3_ops.get_buckets().await;
                            let bucket_names =
                                format!("Available buckets are:\n{:#?}\n", get_bucket_name);

                            let bucket_name = Text::new("Please input the name of the bucket\n")
                                .with_placeholder(&bucket_names)
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .prompt()
                                .unwrap();

                            match bucket_name.is_empty() {
                                false => {
                                    let object_names =
                                        s3_ops.retrieve_keys_in_a_bucket(&bucket_name).await;
                                    for object in object_names {
                                        let colored_key_name = object.green().bold();
                                        println!("    {}\n", colored_key_name);
                                    }
                                }
                                true => {
                                    println!(
                                        "{}\n",
                                        "The bucket name can't be empty; please try again"
                                            .red()
                                            .bold()
                                    )
                                }
                            }
                        }
                        "Delete object from a bucket\n" => {
                            let get_bucket_lists = s3_ops.get_buckets().await;
                            let existing_buckets = format!(
                                "These buckets are already in your account:\n{:#?}\n",
                                get_bucket_lists
                            );
                            let bucket_name = Text::new("Please input the name of the bucket\n")
                                .with_placeholder(&existing_buckets)
                                .with_formatter(&|str| format!(".....{str}....."))
                                .prompt()
                                .unwrap();

                            match bucket_name.is_empty() {
                                false => {
                                    let object_names =
                                        s3_ops.retrieve_keys_in_a_bucket(&bucket_name).await;
                                    let available_object_names = format!(
                                        "The object names are in the {bucket_name} bucket:\n{}\n",
                                        object_names.join("\n")
                                    );
                                    let object_name =
                                        Text::new("Enter the object/key name to delete\n")
                                            .with_placeholder(&available_object_names)
                                            .with_formatter(&|str| format!(".....{str}.....\n"))
                                            .prompt()
                                            .unwrap();
                                    s3_ops
                                        .delete_content_in_a_bucket(&bucket_name, &object_name)
                                        .await;
                                }
                                true => {
                                    println!(
                                        "{}\n",
                                        "The bucket name can't be empty; please try again"
                                            .red()
                                            .bold()
                                    )
                                }
                            }
                        }

                        "Delete Bucket\n" => {
                            let get_bucket_name = s3_ops.get_buckets().await;
                            let bucket_names = format!(
                                "Below, you'll find the buckets in your account:\n{:?}\n",
                                get_bucket_name
                            );
                            let bucket_name = Text::new("Enter the bucket name to delete")
                                .with_placeholder(&bucket_names)
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .with_help_message("These are the buckets within your AWS services")
                                .prompt()
                                .unwrap();
                            match bucket_name.is_empty() {
                                false => {
                                    s3_ops.delete_bucket(&bucket_name).await;
                                }
                                true => {
                                    println!(
                                        "{}\n",
                                        "The bucket name can't be empty; please try again"
                                            .red()
                                            .bold()
                                    )
                                }
                            }
                        }
                        "Put object in a Bucket\n" => {
                            let object = Text::new("Enter the object/data path\n")
                                .with_placeholder(
                                    "You can copy the path and ctrl+shift+v to paste it here without quotation around it",
                                )
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .prompt()
                                .unwrap();

                            let get_bucket_name = s3_ops.get_buckets().await;
                            let available_bucket_name = format!(
                                "Available bucket names in your account:\n {:#?}\n",
                                get_bucket_name
                            );
                            let bucket_name = Text::new("Enter the bucket name\n")
                                .with_placeholder(&available_bucket_name)
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .with_help_message("This is where we put the actual data")
                                .prompt()
                                .unwrap();

                            let key = Text::new("Enter the key or the identifier\n")
                                .with_placeholder("This is what used to retreive the content later")
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .prompt()
                                .unwrap();

                            match (object.is_empty(), bucket_name.is_empty(), key.is_empty()) {
                                (false, false, false) => {
                                    use filesize::PathExt;
                                    use std::path::Path;
                                    let path = Path::new(&object);
                                    let file_size = match path.symlink_metadata() {
                                        Ok(metadata) => match path.size_on_disk_fast(&metadata) {
                                            Ok(realsize) => Some(realsize),
                                            Err(_) => None,
                                        },
                                        Err(_) => None,
                                    };
                                    match file_size {
                                        Some(size) => {
                                            let msg = format!(
                                                "Uploading the file '{}' and its size is: {} Mb\n",
                                                object.green().bold(),
                                                (size / (1024 * 1024)).to_string().green().bold()
                                            );
                                            println!("{msg}");
                                            let size_in_mb = size / (1024 * 1024);
                                            if size_in_mb < 50 {
                                                println!(
                                                    "{}\n",
                                                    "It will take less than a minute to upload the content"
                                                        .yellow()
                                                        .bold()
                                                );
                                            } else if size_in_mb * 1024 < 1048576 {
                                                let guessed_minutes = size_in_mb / (50 * 50);
                                                println!("It will take approximately '{}' minutes to upload content, so please be patient\n",guessed_minutes.to_string().yellow().bold());
                                            } else if size_in_mb * 1024 > 1048576 {
                                                let guessed_hours = size_in_mb / (50 * 50);
                                                if guessed_hours == 0 | 1 {
                                                    println!("It will take approximately '{}' hour to upload content, so you can accomplish other tasks while it's uploading\n",guessed_hours.to_string().yellow().bold());
                                                } else {
                                                    println!("It will take approximately '{}' hours to upload content, so you can accomplish other tasks while it's uploading\n",guessed_hours.to_string().yellow().bold());
                                                }
                                            }
                                        }
                                        None => {
                                            let msg = format!(
                                                "Uploading the file'{}'\n",
                                                object.green().bold()
                                            );
                                            println!("{msg}");
                                            println!("{}\n","No file size information is available; you can either wait or engage in other tasks while the uploading process is in progress".yellow().bold());
                                        }
                                    };
                                    let start_time = std::time::SystemTime::now();
                                    s3_ops
                                        .upload_content_to_a_bucket(&bucket_name, &object, &key)
                                        .await;
                                    let end_time = start_time.elapsed().expect(
                                        "Error while converting to duration from system time\n",
                                    );
                                    if end_time.as_secs() < 60 {
                                        println!(
                                            "It took '{}' seconds to update the file\n",
                                            end_time.as_secs().to_string().yellow().bold()
                                        );
                                    } else if end_time.as_secs() < 36000 {
                                        println!(
                                            "Uploading the provided content required '{}' minutes\n",
                                            (end_time.as_secs() / 60).to_string().yellow().bold()
                                        );
                                    } else {
                                        println!(
                                            "Uploading the provided content required '{}' hours\n",
                                            (end_time.as_secs() / (60 * 60))
                                                .to_string()
                                                .yellow()
                                                .bold()
                                        );
                                    };
                                }

                                _ => {
                                    println!("{}\n", "Data path,the key/object name or the bucket name can't be empty".red().bold())
                                }
                            }
                        }
                        "Modifying Object Visibility\n" => {
                            let get_bucket_name = s3_ops.get_buckets().await;
                            let available_bucket_name = format!(
                                "Available bucket names in your account:\n {:#?}\n",
                                get_bucket_name
                            );
                            let bucket_name = Text::new("Enter bucket name that contains the object to which you want to attach the ACL or Permission\n")
                                .with_placeholder(&available_bucket_name)
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .with_help_message("This is where we put the actual data")
                                .prompt()
                                .unwrap();
                            match bucket_name.is_empty() {
                                false => {
                                    let object_names =
                                        s3_ops.retrieve_keys_in_a_bucket(&bucket_name).await;
                                    let available_object_names = format!(
                                        "The object names are in the {bucket_name} bucket:\n{:#?}\n",
                                        object_names
                                    );
                                    let object_name =
                                        Text::new("Please enter the object name for which you'd like to modify permissions\n")
                                            .with_placeholder(&available_object_names)
                                            .with_formatter(&|str| format!(".....{str}.....\n"))
                                            .prompt()
                                            .unwrap();
                                    let possible_acl_values ="private | public-read | public-read-write | authenticated-read";
                                    let permission_string =
                                        Text::new("Enter the ACL permission strings\n")
                                            .with_placeholder(possible_acl_values)
                                            .with_formatter(&|str| format!(".....{str}.....\n"))
                                            .prompt()
                                            .unwrap();
                                    match (object_name.is_empty(), permission_string.is_empty()) {
                                        (false, false) => {
                                            s3_ops
                                                .put_object_acl(
                                                    &bucket_name,
                                                    &object_name,
                                                    &permission_string,
                                                )
                                                .await;
                                        }
                                        _ => println!("{}\n", "Fields can't be empty".red().bold()),
                                    }
                                }
                                true => println!("{}\n", "Bucket Name Can't be empty".red().bold()),
                            }
                        }

                        "Download object from bucket\n" => {
                            let get_buckets = s3_ops.get_buckets().await;
                            let available_buckets =
                                format!("Available buckets in your account:\n{:#?}\n", get_buckets);

                            let bucket_name = Text::new("Input the bucket name\n")
                                .with_placeholder(&available_buckets)
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .prompt()
                                .unwrap();
                            match bucket_name.is_empty() {
                                false => {
                                    let get_objects =
                                        s3_ops.retrieve_keys_in_a_bucket(&bucket_name).await;
                                    let available_objects = format!(
                                        "Available keys in {bucket_name}\n{:#?}\n",
                                        get_objects
                                    );
                                    let object = Text::new("Input the object/key to download\n")
                                        .with_placeholder(&available_objects)
                                        .with_formatter(&|str| format!(".....{str}.....\n"))
                                        .prompt()
                                        .unwrap();
                                    match object.is_empty() {
                                        false => {
                                            s3_ops
                                                .download_content_from_bcuket(
                                                    &bucket_name,
                                                    &object,
                                                    None,
                                                    true,
                                                )
                                                .await;
                                        }
                                        true => {
                                            println!(
                                                "{}\n",
                                                "key/object name can't be empty".red().bold()
                                            )
                                        }
                                    }
                                }
                                true => {
                                    println!("{}\n", "Bucket name can't be empty".red().bold())
                                }
                            }
                        }

                        "Retrieve a presigned URL for an object\n" => {
                            let get_bucket_name = s3_ops.get_buckets().await;
                            let available_bucket_name = format!(
                                "Available buckets in your account:\n {:?}\n",
                                get_bucket_name
                            );
                            let bucket_name = Text::new("Enter the bucket name\n")
                                .with_placeholder(&available_bucket_name)
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .with_help_message("This is where we put the actual data")
                                .prompt()
                                .unwrap();

                            match bucket_name.is_empty() {
                                false => {
                                    let get_objects =
                                        s3_ops.retrieve_keys_in_a_bucket(&bucket_name).await;
                                    let available_objects = format!(
                                        "Available keys in {bucket_name}\n{:#?}\n",
                                        get_objects
                                    );
                                    let object_name = Text::new("Enter the key or object for which you require a pre-signed URL\n")
                        .with_placeholder(&available_objects)
                        .with_formatter(&|str| format!(".....{str}.....\n"))
                        .prompt()
                        .unwrap();

                                    match object_name.is_empty() {
                                        false => {
                                            let choosing_hour = Text::new("Enter the expiration time for the url in hour\n")
                                    .with_placeholder("Integer values should always be non-negative and should not contain any characters\n")
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt()
                                    .unwrap();
                                            match choosing_hour.is_empty() {
                                                false => {
                                                    let end_time =
                                                        choosing_hour.parse::<u64>().unwrap();
                                                    s3_ops
                                                        .get_presigned_url_for_an_object(
                                                            &bucket_name,
                                                            &object_name,
                                                            end_time,
                                                        )
                                                        .await;
                                                }
                                                true => println!(
                                                    "{}\n",
                                                    "the hour can't be empty".red().bold()
                                                ),
                                            }
                                        }
                                        true => println!(
                                            "{}\n",
                                            "key/object name can't be empty".red().bold()
                                        ),
                                    }
                                }
                                true => {
                                    println!("{}\n", "bucket name can't be empty".red().bold())
                                }
                            }
                        }
                        "Return to the Main Menu\n" => break 's3_ops,
                        _ => {}
                    }
                }
            }

            "Relational Database Service(RDS) Operations\n" => {
                let rds_choices = vec![
                    "Create Db Instance\n",
                    "Default Values\n",
                    "Describe Db Instance\n",
                    "Status of Db Instance\n",
                    "Retrieve Connection URL Information\n",
                    "Start Db Instance\n",
                    "Stop Db Instance\n",
                    "Modify Master Password of Database Instance\n",
                    "Delete Db Instance\n",
                    "Describe Db Cluster\n",
                    "Delete Db Cluster\n",
                    "Return to the Main Menu\n",
                ];

                loop {
                    let choices =
                        Select::new("Select the operations to execute\n", rds_choices.clone())
                            .with_page_size(12)
                            .prompt()
                            .unwrap();
                    match choices {
                        "Create Db Instance\n" => {
                            let db_instance_identifier = Text::new("Enter the database instance identifier\n")
                               .with_placeholder("The DB instance identifier is case-insensitive, but is stored as all lowercase (as in \"mydbinstance\").\nConstraints: 1 to 60 alphanumeric characters or hyphens. First character must be a letter.\n Can't contain two consecutive hyphens. Can't end with a hyphen\n")
                               .with_formatter(&|input| format!("Received Database Instance Identityfier Name: {input}"))
                               .prompt()
                               .unwrap();
                            let engine = Text::new(
                                "Select the database engine for your database instance\n",
                            )
                            .with_placeholder(
                                "Some possible values are: 'mariadb', 'mysql', 'postgres'\n",
                            )
                            .with_formatter(&|input| {
                                format!("Received Database Instance Engine: {input}")
                            })
                            .with_help_message("look here to know more http://tinyurl.com/4h8fcwf6")
                            .prompt()
                            .unwrap();
                            let db_name = Text::new("Select the Database Name for your Database Instance\n")
                                .with_placeholder(
                                    "The interpretation of this parameter varies depending on the chosen database engine\n",
                                )
                                .with_formatter(&|input| format!("Received Database Instance Name: {input}"))
                                .with_help_message(
                                    "Please review this information before providing input: https://tinyurl.com/3ak6pvfs",
                                )
                                .prompt()
                                .unwrap();
                            let storage_type= Text::new("Select the storage type for your Database Instance\n")  
                     .with_placeholder("The storage type and the next database instance class should be a correct combination for successfully creating a database instance\n")
                     .with_formatter(&|input| format!("Received Database Instance Storage Type: {input}"))   
                     .with_help_message("Click here http://tinyurl.com/4h8fcwf6 to learn more") 
                     .prompt()
                     .unwrap();
                            let db_instance_class =  Text::new("Select instance class for your Database Instance\n")  
            .with_placeholder("The instance class and the previous storage type should be a correct combination for successfully creating a database instance\n")
            .with_formatter(&|input| format!("Received Database Instance Class: {input}"))   
            .with_help_message("Click here http://tinyurl.com/29am8kup to learn more") 
            .prompt()
            .unwrap();

                            let allocated_storage = Text::new("Specify the storage capacity for your database in gigabytes(GB), using numerical digits\n")  
            .with_placeholder("The storage requirements depend on your specific use cases and the storage type you have previously selected\n")
            .with_formatter(&|input| format!("Received Storage Allocation for Database Instance : {input}"))   
            .with_help_message("Click here http://tinyurl.com/4h8fcwf6 to learn more") 
            .prompt()
            .unwrap();

                            let username = Text::new("Set the username for your Database Instance\n")  
            .with_placeholder("The username and password options are required parameters for the database instance\n")
            .with_formatter(&|input| format!("Received Database Instance Master Username: {input}"))  
            .prompt()
            .unwrap();
                            let password = inquire::Password::new("Enter the password for your database instance to enable future connectivity\n")
                            .with_display_mode(inquire::PasswordDisplayMode::Masked)
                             .without_confirmation()
                             .with_help_message("The password and preferences will be stored in the current directory for your convenience once the 'Create Database Instance' process is successfully completed")
                             .prompt()
                             .unwrap();

                            match (
                                db_instance_identifier.is_empty(),
                                db_instance_class.is_empty(),
                                storage_type.is_empty(),
                                allocated_storage.is_empty(),
                                db_name.is_empty(),
                                engine.is_empty(),
                                username.is_empty(),
                                password.is_empty(),
                            ) {
                                (false, false, false, false, false, false, false, false) => {
                                    let storage = allocated_storage.parse::<i32>().unwrap();

                                    rds_ops
                                        .create_db_instance(
                                            &db_instance_identifier,
                                            &db_name,
                                            &db_instance_class,
                                            &engine,
                                            &username,
                                            &password,
                                            storage,
                                            &storage_type,
                                        )
                                        .await;

                                    let mut file = OpenOptions::new()
                                        .create(true)
                                        .write(true)
                                        .read(true)
                                        .open("./create_db_instance_choices.txt")
                                        .unwrap();
                                    let choices = format!("Db Instance Identifier: {db_instance_identifier}\nDb Engine: {engine}\nDb Instance Class: {db_instance_class}\nAllocated Storage: {storage}\nStorage Type: {storage_type}\nMaster Username: {username}\nMaster Password: {password}\nDb Name: {db_name}");

                                    match file.write_all(choices.as_bytes()) {
                                        Ok(_) => {
                                            let colored_msg ="The choices have been saved to the current directory for your reference".yellow().bold();
                                            println!("{colored_msg}\n");
                                        }
                                        Err(_) => println!(
                                            "Error while writting file to the current directory\n"
                                        ),
                                    }
                                }
                                _ => {
                                    println!("{}\n", "Fields should not be left empty".red().bold())
                                }
                            }
                        }

                        "Default Values\n" => {
                            let default_cluster_id = rds_ops.get_db_cluster_id().green().bold();
                            let default_db_instance_id =
                                rds_ops.get_db_instance_id().green().bold();
                            println!(
                                "Default Database Instance Identifier: {default_db_instance_id}\n"
                            );
                            println!("Default Database Cluster Identifier: {default_cluster_id}\n");
                            println!("{}\n","If a default value is set, you have the option to omit input for fields where it is required.\n Please ensure to refer to the provided placeholder information".yellow().bold());
                        }

                        "Retrieve Connection URL Information\n" => {
                            let default_db_instance =
                                format!("Default Db Instance Id: {}", rds_ops.get_db_instance_id());
                            let db_instance_identifier =
                                Text::new("Enter the database instance identifier\n")
                                    .with_placeholder(&default_db_instance)
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt_skippable()
                                    .unwrap()
                                    .unwrap();

                            match db_instance_identifier.is_empty() {
                                false => {
                                    let postgres_choice = Confirm::new("Are you in need of a PostgreSQL connection URL?\n")
                             .with_placeholder("yes means ,proceed with the PostgreSQL option, No means you'll receive enough information about the database instance")
                             .prompt()
                             .unwrap();

                                    match postgres_choice {
                                        true => {
                                            let password = Text::new("Enter the password for the chosen database instance\n")  
                              .with_placeholder("Please note that a password is necessary to generate the connection URL for the postgres database\n")
                              .with_formatter(&|str| format!(".....{str}.....\n"))
                              .prompt()
                              .unwrap();
                                            let instance_info = rds_ops
                                                .describe_db_instance(Some(&db_instance_identifier))
                                                .await;
                                            let username = instance_info.get_username();
                                            let endpoint_with_port =
                                                instance_info.get_endpoint_with_port();
                                            let db_name = instance_info.get_db_name();

                                            match (
                                                username,
                                                endpoint_with_port,
                                                db_name,
                                                password.is_empty(),
                                            ) {
                                                (
                                                    Some(username),
                                                    Some(endpoint_with_port),
                                                    Some(db_name),
                                                    false,
                                                ) => {
                                                    let database_url = format!("postgres://{username}:{password}@{endpoint_with_port}/{db_name}").green().bold();
                                                    println!(
                                                        "The database url is: {}\n",
                                                        database_url
                                                    );
                                                    println!("{}\n","Establishing a VPC (Virtual Private Cloud) and configuring the appropriate security group for the database instance is essential to ensure successful connectivity to the database instance".yellow().bold());
                                                    let status_ = rds_ops
                                                        .status_of_db_instance(Some(
                                                            &db_instance_identifier,
                                                        ))
                                                        .await;
                                                    if let Some(status) = status_ {
                                                        let colored_status = status.green().bold();
                                                        println!(
                                                            "{}: {colored_status}\n",
                                                            "The current status of Db Instance"
                                                                .yellow()
                                                                .bold()
                                                        );
                                                    }
                                                }
                                                _ => {
                                                    println!(
                                                        "{}\n",
                                                        "Database URL cannot be generated"
                                                            .yellow()
                                                            .bold()
                                                    );
                                                    println!("{}\n","Please verify the status of the DB instance by selecting the 'Status of DB Instance' option".yellow().bold());
                                                }
                                            }
                                        }
                                        false => {
                                            let instance_info = rds_ops
                                                .describe_db_instance(Some(&db_instance_identifier))
                                                .await;
                                            let username = instance_info.get_username();
                                            let endpoint_with_port =
                                                instance_info.get_endpoint_with_port();
                                            let db_name = instance_info.get_db_name();

                                            match (username, endpoint_with_port, db_name) {
                                                (
                                                    Some(username),
                                                    Some(endpoint_with_port),
                                                    Some(db_name),
                                                ) => {
                                                    let colored_username = username.blue().bold();
                                                    let colored_endpoint_with_port =
                                                        endpoint_with_port.blue().bold();
                                                    let colored_db_name = db_name.blue().bold();
                                                    println!("Username: {colored_username}");
                                                    println!("Endpoint with port: {colored_endpoint_with_port}");
                                                    println!("Db Name: {colored_db_name}");
                                                    let status_ = rds_ops
                                                        .status_of_db_instance(Some(
                                                            &db_instance_identifier,
                                                        ))
                                                        .await;
                                                    if let Some(status) = status_ {
                                                        let colored_status = status.green().bold();
                                                        println!(
                                                            "{}: {colored_status}\n",
                                                            "The current status of Db Instance"
                                                                .yellow()
                                                                .bold()
                                                        );
                                                    }
                                                }
                                                _ => println!("Database url can't be generated\n"),
                                            }
                                        }
                                    }
                                }

                                true => {
                                    let postgres_choice = Confirm::new("Are you in need of a PostgreSQL connection URL?\n")
                                    .with_placeholder("yes means ,proceed with the PostgreSQL option, No means you'll receive enough information about the database instance")
                                    .prompt()
                                    .unwrap();

                                    match postgres_choice {
                                        true => {
                                            let password = Text::new("Enter the password for the chosen database instance\n")  
                                     .with_placeholder("Please note that a password is necessary to generate the connection URL for the postgres database\n")
                                     .with_formatter(&|str| format!(".....{str}.....\n"))
                                     .prompt()
                                     .unwrap();
                                            let instance_info =
                                                rds_ops.describe_db_instance(None).await;
                                            let username = instance_info.get_username();
                                            let endpoint_with_port =
                                                instance_info.get_endpoint_with_port();
                                            let db_name = instance_info.get_db_name();

                                            match (
                                                username,
                                                endpoint_with_port,
                                                db_name,
                                                password.is_empty(),
                                            ) {
                                                (
                                                    Some(username),
                                                    Some(endpoint_with_port),
                                                    Some(db_name),
                                                    false,
                                                ) => {
                                                    let database_url = format!("postgres://{username}:{password}@{endpoint_with_port}/{db_name}").green().bold();
                                                    println!(
                                                        "The database url is: {}\n",
                                                        database_url
                                                    );
                                                    let status_ =
                                                        rds_ops.status_of_db_instance(None).await;
                                                    if let Some(status) = status_ {
                                                        let colored_status = status.green().bold();
                                                        println!(
                                                            "{}: {colored_status}\n",
                                                            "The current status of Db Instance"
                                                                .yellow()
                                                                .bold()
                                                        );
                                                    }
                                                }
                                                _ => println!("Database url can't be generated\n"),
                                            }
                                        }
                                        false => {
                                            let instance_info = rds_ops
                                                .describe_db_instance(Some(&db_instance_identifier))
                                                .await;
                                            let username = instance_info.get_username();
                                            let endpoint_with_port =
                                                instance_info.get_endpoint_with_port();
                                            let db_name = instance_info.get_db_name();

                                            match (username, endpoint_with_port, db_name) {
                                                (
                                                    Some(username),
                                                    Some(endpoint_with_port),
                                                    Some(db_name),
                                                ) => {
                                                    let colored_username = username.blue().bold();
                                                    let colored_endpoint_with_port =
                                                        endpoint_with_port.blue().bold();
                                                    let colored_db_name = db_name.blue().bold();
                                                    println!("Username: {colored_username}");
                                                    println!("Endpoint with port: {colored_endpoint_with_port}");
                                                    println!("Db Name: {colored_db_name}");
                                                    let status_ = rds_ops
                                                        .status_of_db_instance(Some(
                                                            &db_instance_identifier,
                                                        ))
                                                        .await;
                                                    if let Some(status) = status_ {
                                                        let colored_status = status.green().bold();
                                                        println!(
                                                            "{}: {colored_status}\n",
                                                            "The current status of Db Instance"
                                                                .yellow()
                                                                .bold()
                                                        );
                                                    }
                                                }
                                                _ => println!("Database url can't be generated\n"),
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        "Describe Db Instance\n" => {
                            let default_db_instance =
                                format!("Default Db Instance Id: {}", rds_ops.get_db_instance_id());
                            let db_instance_identifier =
                                Text::new("Enter the database instance identifier\n")
                                    .with_placeholder(&default_db_instance)
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt_skippable()
                                    .unwrap()
                                    .unwrap();

                            let instance_info = match db_instance_identifier.is_empty() {
                                false => {
                                    rds_ops
                                        .describe_db_instance(Some(&db_instance_identifier))
                                        .await
                                }

                                true => rds_ops.describe_db_instance(None).await,
                            };
                            let endpoint_with_port = instance_info.get_endpoint_with_port();
                            let zone = instance_info.get_availability_zone();
                            let class = instance_info.get_instance_class();
                            let db_name = instance_info.get_db_name();
                            let status = instance_info.get_instance_status();
                            if let Some(endpoint_with_port_) = endpoint_with_port {
                                println!(
                                    "Endpoint_With_Port: {}",
                                    endpoint_with_port_.green().bold()
                                );
                            }
                            if let Some(zone_) = zone {
                                println!("Zone: {}", zone_.green().bold());
                            }
                            if let Some(class_) = class {
                                println!("Instance Class: {}", class_.green().bold());
                            }
                            if let Some(db_name_) = db_name {
                                println!("Database Name(db name): {}", db_name_.green().bold());
                            }
                            if let Some(status_) = status {
                                println!("Status of Db Instance: {}\n", status_.green().bold());
                            }
                        }

                        "Start Db Instance\n" => {
                            let default_instance_id = match var("DB_INSTANCE_ID") {
                                Ok(_) => {
                                    let db_status = rds_ops
                                        .status_of_db_instance(None)
                                        .await
                                        .unwrap_or("Can't get Db Instance Status".into());
                                    format!("The default instance ID: {} and the Current Status Of Db Instance: {}\n",rds_ops.get_db_instance_id(),db_status)
                                }
                                Err(_) => {
                                    format!(
                                        "The Default Instance(DB) ID: {}",
                                        rds_ops.get_db_instance_id()
                                    )
                                }
                            };
                            let db_instance_identifier =
                                Text::new("Enter the database instance identifier\n")
                                    .with_placeholder(&default_instance_id)
                                    .with_help_message("The status of the DB instance should be \"stopped\"; otherwise, this operation will result in a panic (the Rust way of handling runtime exceptions)")
                                     .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt_skippable()
                                    .unwrap()
                                    .unwrap();
                            match db_instance_identifier.is_empty() {
                                false => {
                                    rds_ops
                                        .start_db_instance(Some(&db_instance_identifier))
                                        .await
                                }
                                true => rds_ops.start_db_instance(None).await,
                            }
                        }

                        "Stop Db Instance\n" => {
                            let default_instance_id = match var("DB_INSTANCE_ID") {
                                Ok(_) => {
                                    let db_status = rds_ops
                                        .status_of_db_instance(None)
                                        .await
                                        .unwrap_or("Can't get Db Instance Status".into());
                                    format!("The default instance ID: {} and the Current Status Of Db Instance: {}\n",rds_ops.get_db_instance_id(),db_status)
                                }
                                Err(_) => {
                                    format!(
                                        "The Default Instance(DB) ID: {}",
                                        rds_ops.get_db_instance_id()
                                    )
                                }
                            };
                            let confirm = Confirm::new("Please respond 'Yes' to proceed with stopping the instance, or 'No' to perform the 'Status Of DB Instance' operation\n")
                            .with_placeholder(&default_instance_id)
                            .with_help_message("The DB instance's status should be 'available'; otherwise,\nthis operation may lead to a panic, which is Rust's way of handling runtime exceptions")
                            .prompt()
                            .unwrap();
                            match confirm {
                                false => println!("{}\n", "Okay Sure".green().bold()),
                                true => {
                                    let db_instance_identifier = Text::new("Enter the database instance identifier for which you want to stop temporarily\n")  
                            .with_placeholder(&default_instance_id)
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .with_help_message("This operation assumes that you already know the status of the database instance is 'available'")
                            .prompt_skippable()
                            .unwrap()
                            .unwrap();
                                    match db_instance_identifier.is_empty() {
                                        false => {
                                            rds_ops
                                                .stop_db_instance(Some(&db_instance_identifier))
                                                .await
                                        }
                                        true => rds_ops.stop_db_instance(None).await,
                                    }
                                }
                            }
                        }
                        "Modify Master Password of Database Instance\n" => {
                            let default_db_instance =
                                format!("Default Db Instance Id: {}", rds_ops.get_db_instance_id());
                            let db_instance_identifier =
                                Text::new("Please provide the DB instance ID for which you would like to modify the password\n")
                                    .with_placeholder(&default_db_instance)
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt_skippable()
                                    .unwrap()
                                    .unwrap();
                            let master_password = Text::new("Enter the new master password to replace the old one\n")
                                    .with_placeholder("Please remember this password, as it is used to connect to various database instances\n")
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt()
                                    .unwrap();
                            let apply = Confirm::new("Would you like to apply the changes immediately, or would you prefer to have Amazon Web Services do it later?\n")
                                .with_placeholder("Select 'Yes' to apply immediately or 'No' to have it done later by AWS")
                                .prompt()
                                .unwrap();
                            match (
                                db_instance_identifier.is_empty(),
                                master_password.is_empty(),
                            ) {
                                (false, false) => {
                                    rds_ops
                                        .modify_db_instance(
                                            &db_instance_identifier,
                                            &master_password,
                                            apply,
                                        )
                                        .await;
                                }
                                (true, false) => {
                                    let default_db_instance = format!(
                                        "Default Db Instance Id: {}",
                                        rds_ops.get_db_instance_id()
                                    );
                                    rds_ops
                                        .modify_db_instance(
                                            &default_db_instance,
                                            &master_password,
                                            apply,
                                        )
                                        .await;
                                }
                                _ => {
                                    println!(
                                        "{}\n",
                                        "Password Field can't be left empty".red().bold()
                                    )
                                }
                            }
                        }

                        "Delete Db Instance\n" => {
                            let default_instance_id = match var("DB_INSTANCE_ID") {
                                Ok(_) => {
                                    let db_status = rds_ops
                                        .status_of_db_instance(None)
                                        .await
                                        .unwrap_or("Can't get Db Instance Status".into());
                                    format!("The default instance ID: {} and the Current Status Of Db Instance: {}\n",rds_ops.get_db_instance_id(),db_status)
                                }
                                Err(_) => {
                                    format!(
                                        "The Default Instance(DB) ID: {}",
                                        rds_ops.get_db_instance_id()
                                    )
                                }
                            };
                            let db_instance_identifier = Text::new("Enter the database instance identifier you wish to delete permanently\n")  
                            .with_placeholder(&default_instance_id)
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .prompt_skippable()
                            .unwrap()
                            .unwrap();
                            match db_instance_identifier.is_empty() {
                                false => {
                                    rds_ops
                                        .delete_db_instance(Some(&db_instance_identifier))
                                        .await
                                }

                                true => rds_ops.delete_db_instance(None).await,
                            }
                        }

                        "Status of Db Instance\n" => {
                            let default_db_instance =
                                format!("Default Db Instance Id: {}", rds_ops.get_db_instance_id());
                            let db_instance_identifier =
                                Text::new("Enter the database instance identifier\n")
                                    .with_placeholder(&default_db_instance)
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt_skippable()
                                    .unwrap()
                                    .unwrap();

                            match db_instance_identifier.is_empty() {
                                false => {
                                    let status_ = rds_ops
                                        .status_of_db_instance(Some(&db_instance_identifier))
                                        .await;
                                    if let Some(status) = status_ {
                                        let colored_status = status.green().bold();
                                        println!(
                                            "{}: {colored_status}\n",
                                            "The current Status of Db Instance".yellow().bold()
                                        );
                                    }
                                }
                                true => {
                                    let status_ = rds_ops.status_of_db_instance(None).await;
                                    if let Some(status) = status_ {
                                        let colored_status = status.green().bold();
                                        println!(
                                            "{}: {colored_status}\n",
                                            "The current Status of Db Instance".yellow().bold()
                                        );
                                    }
                                }
                            }
                        }
                        "Describe Db Cluster\n" => {
                            let default_cluster_id = format!(
                                "The default cluster ID: {}\n",
                                rds_ops.get_db_cluster_id()
                            );
                            let db_cluster_identifier = Text::new("Enter the database cluster identifier, which is different from the database instance identifier\n")  
                             .with_placeholder(&default_cluster_id)
                             .with_formatter(&|str| format!(".....{str}.....\n"))
                              .prompt_skippable()
                               .unwrap()
                               .unwrap();
                            let cluster_info = match db_cluster_identifier.is_empty() {
                                false => {
                                    rds_ops
                                        .describe_db_cluster(Some(&db_cluster_identifier))
                                        .await
                                }
                                true => rds_ops.describe_db_cluster(None).await,
                            };

                            cluster_info.into_iter().for_each(|dbclusterinfo| {
                                let status = dbclusterinfo.get_status();
                                let instance_members = dbclusterinfo.get_db_members();
                                let cluster_endpoint_with_port =
                                    dbclusterinfo.get_cluster_endpoint_with_port();
                                let master_user_name = dbclusterinfo.get_master_username();
                                let cluster_db_name = dbclusterinfo.get_db_name();
                                if let Some(status) = status {
                                    let colored_status = status.green().bold();
                                    println!("Current Status of Cluster: {colored_status}");
                                }
                                if let Some(cluster_endpoint) = cluster_endpoint_with_port {
                                    let colored_endpoint = cluster_endpoint.green().bold();
                                    println!("Cluster endpoint with port: {colored_endpoint}");
                                }
                                if let Some(master_username) = master_user_name {
                                    let colored_username = master_username.green().bold();
                                    println!("Master Username of the Cluster: {colored_username}");
                                }
                                if let Some(db_name) = cluster_db_name {
                                    let colored_dbname = db_name.green().bold();
                                    println!("Cluster Database Name: {colored_dbname}\n");
                                }
                                println!("{}\n", "Db Instances info:".blue().bold());
                                instance_members.into_iter().for_each(|db_instance_info| {
                                    let colored_id = db_instance_info.green().bold();
                                    println!("{colored_id}\n");
                                })
                            });
                        }
                        "Delete Db Cluster\n" => {
                            let default_cluster_id = format!(
                                "The default cluster ID: {}\n",
                                rds_ops.get_db_cluster_id()
                            );
                            let db_cluster_identifier = Text::new("Enter the database cluster identifier, which is different from the database instance identifier\n")  
                             .with_placeholder(&default_cluster_id)
                             .with_formatter(&|str| format!(".....{str}.....\n"))
                             .prompt_skippable()
                             .unwrap()
                             .unwrap();
                            let cluster_info = match db_cluster_identifier.is_empty() {
                                false => {
                                    rds_ops
                                        .delete_db_cluster(Some(&db_cluster_identifier))
                                        .await
                                }
                                true => rds_ops.delete_db_cluster(None).await,
                            };

                            let colored_status = cluster_info
                                .get_status()
                                .map(|status| status.green().bold());
                            let instance_members = cluster_info.get_db_members();
                            println!("Status of Db Cluster: {:?}\n", colored_status);
                            println!("{}\n", "Db Instances info".blue().bold());
                            instance_members.into_iter().for_each(|db_instance_id| {
                                let colored_instance_id = db_instance_id.green().bold();
                                println!("Db Instance Id: {}\n", colored_instance_id);
                            });
                        }
                        "Return to the Main Menu\n" => continue 'main,

                        _ => println!("Never reach"),
                    }
                }
            }

            "MemoryDb Operations\n" => {
                let memdb_choices = vec![
                    "Create Access Control List (ACL) for user permissions\n",
                    "Create MemDb Cluster\n",
                    "Create MemDb User\n",
                    "View ACL Details\n",
                    "Describe MemDb Cluster\n",
                    "Describe MemDb User\n",
                    "Describe Snapshots of MemDb Cluster\n",
                    "Retrieve the database URL for connection\n",
                    "Delete Access Control List (ACL)\n",
                    "Delete MemDb User\n",
                    "Delete Cluster\n",
                    "Return to the Main Menu\n",
                ];

                loop {
                    let choices =
                        Select::new("Select the operations to execute\n", memdb_choices.clone())
                            .with_page_size(12)
                            .prompt()
                            .unwrap();

                    match choices {
                        "Create Access Control List (ACL) for user permissions\n" => {
                            let acl_name = Text::new(
                                "Please enter the name for the new ACL you want to create\n",
                            )
                            .with_placeholder("The name must be uniquely identifiable")
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .prompt()
                            .unwrap();

                            match acl_name.is_empty() {
                                false => {
                                    memdb_ops.create_acl(&acl_name).await;
                                }
                                true => {
                                    println!("{}\n", "ACL name cannot be left empty.".red().bold())
                                }
                            }
                        }

                        "Create MemDb Cluster\n" => {
                            let cluster_name = Text::new("Enter the cluster name\n")
                                .with_placeholder("The name must be uniquely identifiable")
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .prompt()
                                .unwrap();
                            let possible_values = vec![
                                "db.t4g.small",
                                "db.r6g.large",
                                "db.r6g.xlarge",
                                "db.r6g.2xlarge",
                            ];
                            let possible_values =
                                format!("Some possible Values are: {:#?}\n", possible_values);
                            let node_type =
                                Text::new("Select the node type for your database system\n")
                                    .with_placeholder(&possible_values)
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .with_help_message(
                                        "look here to know more https://tinyurl.com/axy83wff",
                                    )
                                    .prompt()
                                    .unwrap();

                            let acl_name = Text::new("Specify the name of the Access Control List (ACL) to associate with the cluster\n")
                        .with_placeholder("Acl name is created through the aws console of memdb.")
                        .with_formatter(&|str| format!(".....{str}.....\n"))
                        .with_help_message("look here to know more https://tinyurl.com/yn3n4wya")
                        .prompt()
                        .unwrap();

                            match (
                                cluster_name.is_empty(),
                                node_type.is_empty(),
                                acl_name.is_empty(),
                            ) {
                                (false, false, false) => {
                                    memdb_ops
                                        .create_memdb_cluster(&node_type, &cluster_name, &acl_name)
                                        .await;
                                }
                                _ => {
                                    println!("{}\n", "Fields should not be left empty".red().bold())
                                }
                            }
                        }
                        "Create MemDb User\n" => {
                            let user_name = Text::new("Please provide a name for this MemDB user\n")
                        .with_placeholder("This name will also serve as the username for the database within a MemDB cluster\n")
                        .with_formatter(&|str| format!(".....{str}.....\n"))
                        .prompt()
                        .unwrap();
                            let possible_access_string_values = "The formats\n 'on' -The user is an active user\n '~*' - Access is given to all available keys\n '+@all' - Access is given to all available commands\n";
                            let access_string = Text::new("Please provide the access string or permission values for this user\n")
                                       .with_placeholder(possible_access_string_values)
                                       .with_formatter(&|str| format!(".....{str}.....\n"))
                                       .with_help_message("Look here to know more https://tinyurl.com/2p9mnm64")
                                       .prompt()
                                       .unwrap();
                            let possible_authenticated_types =
                                "    iam or Iam\n    Password or password\n";
                            let auth_type = Text::new("Specify the authenticated user's type\n")
                                .with_placeholder(possible_authenticated_types)
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .with_help_message(
                                    "Look here to know more https://tinyurl.com/3zaztx97",
                                )
                                .prompt()
                                .unwrap();
                            let passwords = Text::new("Please enter the passwords for the memdb user\n")
                                     .with_placeholder("Please remember this password; it's used for authenticating the database in a 'memdb' cluster\n")
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                     .with_help_message("Please ensure that your password contains a minimum of 16 characters")
                                     .prompt()
                                     .unwrap();
                            match (
                                user_name.is_empty(),
                                access_string.is_empty(),
                                auth_type.is_empty(),
                                passwords.is_empty(),
                            ) {
                                (false, false, false, false) => {
                                    memdb_ops
                                        .create_memdb_user(
                                            &user_name,
                                            &access_string,
                                            &auth_type,
                                            &passwords,
                                        )
                                        .await;
                                    let mut file = OpenOptions::new()
                                        .create(true)
                                        .write(true)
                                        .read(true)
                                        .open("./create_memdb_user_choices.txt")
                                        .unwrap();
                                    let choices = format!("Memdb User Name: {user_name}\nAccess String value: {access_string}\nAuthentication Type: {auth_type}\nPasswords: {passwords}\n");

                                    match file.write_all(choices.as_bytes()) {
                                        Ok(_) => {
                                            let colored_msg ="The choices have been saved to the current directory for your reference\n".green().bold();
                                            println!("{colored_msg}");
                                        }
                                        Err(_) => println!(
                                            "Error while writting file to the current directory\n"
                                        ),
                                    }
                                }
                                _ => {
                                    println!("{}\n", "Fields should not be left empty".red().bold())
                                }
                            }
                        }

                        "View ACL Details\n" => {
                            let get_acl_names = memdb_ops.describe_acls().await;
                            let mut acl_names = Vec::new();
                            get_acl_names.into_iter().for_each(|acl_infos| {
                                let acl_name = acl_infos.get_acl_name();
                                if let Some(acl_name_) = acl_name {
                                    acl_names.push(acl_name_);
                                }
                            });
                            let available_acl_names = format!("List of Access Control List (ACL) Names in Your Credentials:\n {:#?}\n",acl_names);
                            let acl_name = Text::new(
                                "Please enter the ACL name for the information you seek\n",
                            )
                            .with_placeholder(&available_acl_names)
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .prompt()
                            .unwrap();
                            match acl_name.is_empty() {
                                false => {
                                    let acl_info = memdb_ops.describe_acl(&acl_name).await;

                                    if let (Some(status), Some(user_names), Some(clusters)) = (
                                        acl_info.get_status_of_acl(),
                                        acl_info.get_user_names(),
                                        acl_info.get_clusters(),
                                    ) {
                                        let colored_status = status.green().bold();
                                        println!("The current status of ACL: {}\n", colored_status);
                                        println!(
                                            "{}\n",
                                            "Usernames in an Access Control List (ACL)"
                                                .blue()
                                                .bold()
                                        );
                                        user_names.into_iter().for_each(|user_name| {
                                            let colored_user_name = user_name.green().bold();
                                            println!("{}\n", colored_user_name);
                                        });
                                        println!(
                                            "{}\n",
                                            "Clusters in an Access Control List(ACL)".blue().bold()
                                        );
                                        clusters.into_iter().for_each(|cluster| {
                                            let colored_cluster_name = cluster.green().bold();
                                            println!("{}\n", colored_cluster_name);
                                        });
                                    }
                                }
                                true => {
                                    println!("{}\n", "ACL name cannot be left empty.".red().bold())
                                }
                            }
                        }

                        "Describe MemDb Cluster\n" => {
                            let cluster_name = Text::new("Enter the cluster name for which you want to retrieve information\n")
                        .with_placeholder("The cluster anem is generated during the MemDB cluster creation process\n")
                        .with_formatter(&|str| format!(".....{str}.....\n"))
                        .prompt()
                        .unwrap();

                            match cluster_name.is_empty() {
                                false => {
                                    let info =
                                        memdb_ops.describe_memdb_cluster(&cluster_name).await;
                                    info.into_iter()
                                .for_each(|memclusterinfo|{
                                     let status = memclusterinfo.get_status().unwrap().green().bold();
                                     let acl_name = memclusterinfo.get_acl_name().unwrap().green().bold();
                                     println!("Status of MemdbCluster: {}\nAccess Control List(ACL) name: {}\n",status,acl_name);
                                });
                                }

                                true => println!(
                                    "{}\n",
                                    "The cluster name field can't be empty".red().bold()
                                ),
                            }
                        }
                        "Describe MemDb User\n" => {
                            let username = Text::new("Enter the MemDB user name for which you want to retrieve information\n")
             .with_placeholder("The username is generated during the MemDB user creation process\n")
             .with_formatter(&|str| format!(".....{str}.....\n"))
            .prompt()
            .unwrap();
                            match username.is_empty() {
                                false => {
                                    let user_info = memdb_ops.describe_memdb_user(&username).await;
                                    let status = user_info[0].get_status().take();
                                    let access_string = user_info[0].get_access_string().take();
                                    println!("Status of User: {status:?}\n");
                                    println!("Access String for the User: {access_string:?}\n");
                                    user_info[0].print_auth_info();
                                }
                                true => {
                                    println!("{}\n", "Fields should not be left empty".red().bold())
                                }
                            }
                        }

                        "Describe Snapshots of MemDb Cluster\n" => {
                            let cluster_name = Text::new(
                                "Enter the cluster name for which you want to get snapshots\n",
                            )
                            .with_placeholder("The cluster name is generated during the MemDB cluster creation process\n")
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .prompt()
                            .unwrap();

                            match cluster_name.is_empty() {
                                false => {
                                    let snapshots =
                                        memdb_ops.describe_snapshots(&cluster_name).await;
                                    snapshots.into_iter()
                    .for_each(|snapshot|{
                        let snapshot_name = snapshot.name();
                        let snapshot_status = snapshot.status();
                        println!("Snapshot Name: {snapshot_name:?}\nStatus of snapshot: {snapshot_status:?}\n");
                    });
                                }
                                true => {
                                    println!("{}\n", "Cluster name can't be empty".red().bold())
                                }
                            }
                        }

                        "Retrieve the database URL for connection\n" => {
                            let cluster_name = Text::new("Enter the cluster name for which you need the connection URL\n")
                        .with_placeholder("The cluster name is the name assigned to the cluster when it was initially created\n")
                        .with_formatter(&|str| format!(".....{str}.....\n"))
                        .prompt()
                        .unwrap();

                            match cluster_name.is_empty() {
                                false => {
                                    let info =
                                        memdb_ops.describe_memdb_cluster(&cluster_name).await;
                                    let endpoint_with_port = info[0].get_endpoint_with_port();
                                    if let Some(endpoint_port) = endpoint_with_port {
                                        let redis_url =
                                            format!("redis://{endpoint_port}").green().bold();
                                        println!("The redis database url is: {redis_url}\n");
                                    }
                                }
                                true => println!(
                                    "{}\n",
                                    "MemdDb cluster name can't be empty".red().bold()
                                ),
                            }
                        }

                        "Delete Access Control List (ACL)\n" => {
                            let get_acl_names = memdb_ops.describe_acls().await;
                            let mut acl_names = Vec::new();
                            get_acl_names.into_iter().for_each(|acl_infos| {
                                let acl_name = acl_infos.get_acl_name();
                                if let Some(acl_name_) = acl_name {
                                    acl_names.push(acl_name_);
                                }
                            });
                            let available_acl_names = format!("List of Access Control List (ACL) Names in Your Credentials:\n{:#?}",acl_names);
                            let acl_name = Text::new(
                                "Please provide the name of the ACL you wish to delete\n",
                            )
                            .with_placeholder(&available_acl_names)
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .prompt()
                            .unwrap();
                            match acl_name.is_empty() {
                                false => {
                                    let acl_info = memdb_ops.delete_acl(&acl_name).await;

                                    if let (Some(status), Some(user_names), Some(clusters)) = (
                                        acl_info.get_status_of_acl(),
                                        acl_info.get_user_names(),
                                        acl_info.get_clusters(),
                                    ) {
                                        let colored_status = status.green().bold();
                                        println!("The current status of ACL: {}\n", colored_status);
                                        println!(
                                            "{}\n",
                                            "Usernames in an Access Control List (ACL)"
                                                .blue()
                                                .bold()
                                        );
                                        user_names.into_iter().for_each(|user_name| {
                                            let colored_user_name = user_name.green().bold();
                                            println!("{}\n", colored_user_name);
                                        });
                                        println!(
                                            "{}\n",
                                            "Clusters in an Access Control List(ACL)".blue().bold()
                                        );
                                        clusters.into_iter().for_each(|cluster| {
                                            let colored_cluster_name = cluster.green().bold();
                                            println!("{}\n", colored_cluster_name);
                                        });
                                    }
                                }
                                true => println!("{}\n", "ACL name can't be empty".red().bold()),
                            }
                        }

                        "Delete MemDb User\n" => {
                            let username = Text::new("Enter the MemDB user name to delete\n") 
                          .with_placeholder("The username is generated during the MemDB user creation process\n")
                          .with_formatter(&|str| format!(".....{str}.....\n"))
                           .prompt()
                           .unwrap();
                            match username.is_empty() {
                                false => memdb_ops.delete_memdb_user(&username).await,
                                true => println!("{}\n", "User name can't be empty".red().bold()),
                            }
                        }
                        "Delete Cluster\n" => {
                            let cluster_name =
                                Text::new("Enter the cluster name for which you want to delete\n")
                                     .with_placeholder("The cluster name is generated during the MemDB cluster creation process\n")
                                     .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt()
                                    .unwrap();
                            let final_snapshot_name =
                                Text::new("Create snapsho\n")
                                    .with_placeholder("You can create a final snapshot of your cluster before it’s deleted so you can restore it later\n")
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt()
                                    .unwrap();

                            match (cluster_name.is_empty(), final_snapshot_name.is_empty()) {
                                (false, false) => {
                                    memdb_ops
                                        .delete_memdb_cluster(&cluster_name, &final_snapshot_name)
                                        .await;
                                }
                                _ => {
                                    println!("{}\n", "Fields should not be left empty".red().bold())
                                }
                            }
                        }
                        "Return to the Main Menu\n" => continue 'main,
                        _ => println!("Never reach"),
                    }
                }
            }

            "Quit the application\n" => {
                credential.empty();
                break 'main;
            }
            _other => {
                println!("This branch never reach..");
            }
        }
    }
}
fn global_render_config() -> RenderConfig {
    let mut config = RenderConfig::default()
        .with_prompt_prefix(Styled::new("⚙️").with_fg(inquire::ui::Color::DarkBlue))
        .with_text_input(StyleSheet::new().with_fg(inquire::ui::Color::LightGreen))
        .with_highlighted_option_prefix(Styled::new("👉"))
        .with_help_message(StyleSheet::new().with_fg(inquire::ui::Color::DarkYellow));
    config.answer = StyleSheet::new()
        .with_attr(Attributes::BOLD)
        .with_fg(inquire::ui::Color::DarkGreen);
    config
}
