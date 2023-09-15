//Foreign crates
use colored::Colorize;
use inquire::{
    ui::{Attributes, RenderConfig, StyleSheet, Styled},
    Confirm, Select, Text,
};

use std::fs::OpenOptions;
use std::io::Write;

//Interneal APIs

use aws_apis::{
    load_credential_from_env, CredentInitialize, FaceDetails, MemDbOps, PollyOps, RdsOps,
    RekognitionOps, S3Ops, SesOps, SimpleMail, Simple_, SnsOps, TemplateMail, Template_,
    TextDetect,
};

use dotenv::dotenv;
use std::env::var;
#[tokio::main]
async fn main() {
    inquire::set_global_render_config(global_render_config());

    let operations: Vec<&str> = vec![
        "Verify the Credential\n",
        "Print Credentials Information\n",
        "Amazon Polly Operations\n",
        "Amazon Rekognition Operations\n",
        "AWS Simple Notification Service(SNS) Operations\n",
        "AWS Simple Email Service(SES) Operations\n",
        "S3 Bucket Operations\n",
        "Relational Database Service(RDS) Operations\n",
        "MemoryDb Operations\n",
        "Quit the application\n",
    ];
    //Intial dummy credentials
    let mut credential = CredentInitialize::default();
    //Using same credentials for the different services.
    let mut ses_ops: SesOps = SesOps::build(credential.build());
    let mut s3_ops: S3Ops = S3Ops::build(credential.build());
    let mut rds_ops: RdsOps = RdsOps::build(credential.build());
    let mut memdb_ops: MemDbOps = MemDbOps::build(credential.build());
    let mut polly_ops: PollyOps = PollyOps::build(credential.build());
    let mut sns_ops: SnsOps = SnsOps::build(credential.build());
    let mut rekognition_ops: RekognitionOps = RekognitionOps::build(credential.build());

    'main: loop {
        let choice = Select::new("Select the option to execute the operation\n", operations.clone())
            .with_help_message("Don't enclose data in quotation marks or add spaces around it in any operations,\nexcept when working with template data.")
            .with_page_size(10)
            .prompt()
            .unwrap();

        match choice {
            "Verify the Credential\n" => {
                let choices = Confirm::new("Load the credentials from the configuration file or from environment variables\n")
                          .with_placeholder("Use 'Yes' to load from the environment and 'No' to load from environment variables\n")
                          .with_help_message("message")
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
                        polly_ops = PollyOps::build(config.clone());
                        sns_ops = SnsOps::build(config.clone());
                        rekognition_ops = RekognitionOps::build(config);
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
                        polly_ops = PollyOps::build(config.clone());
                        sns_ops = SnsOps::build(config.clone());
                        rekognition_ops = RekognitionOps::build(config);
                        println!("{}\n","Please verify the credentials by printing the credential information before proceeding with any operations".red().bold());
                    }
                }
            }
            "Print Credentials Information\n" => {
                let confirm =
                    Confirm::new("Are you sure you want to print credential information?\n")
                        .with_formatter(&|str| format!(".....{str}.....\n"))
                        .with_placeholder("This is solely for verification purposes\n")
                        .with_default(false)
                        .prompt()
                        .unwrap();

                match confirm {
                    true => {
                        println!("Here is your credential informations\n");
                        println!("{:#?}", credential.get_credentials());
                    }
                    false => {
                        println!("{}\n", "Sure...".green().bold())
                    }
                }
            }

            "Amazon Polly Operations\n" => {
                let polly_operations = vec![
                    "Start Synthesizing Speech Task\n",
                    "List Speech Synthesis Tasks\n",
                    "Retrieve voice information from Amazon Polly\n",
                    "Go To Main Menu\n",
                ];

                loop {
                    let polly_choices = Select::new(
                        "Select the option to execute the operation\n",
                        polly_operations.clone(),
                    )
                    .with_help_message("Do not enclose it with quotation marks or add spaces")
                    .with_vim_mode(true)
                    .with_page_size(4)
                    .prompt()
                    .unwrap();
                    match polly_choices {
                        "Start Synthesizing Speech Task\n" => {
                            let possible_engines =
                                "Possible Engine Values are: 'standard'\n'neural'\n";
                            let engine_name =
                                Text::new("Select the speech generation engine name\n")
                                    .with_placeholder(possible_engines)
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt()
                                    .unwrap();
                            match engine_name.is_empty() {
                                false => {
                                    let (voice_ids, lang_codes) =
                                        polly_ops.get_voice_info_given_engine(&engine_name).await;
                                    let mut vec_of_voice_ids = Vec::new();
                                    voice_ids.into_iter().for_each(|voice_id| {
                                        if let Some(voiceid) = voice_id {
                                            vec_of_voice_ids.push(voiceid.as_str().to_owned());
                                        }
                                    });
                                    let available_voiceid_specified_engine = format!("Voice ID's for the specified engine: {engine_name}\n{:?}\n",vec_of_voice_ids.join(" | "));
                                    let voice_id = Text::new(
                                        "Select the voice for audio generation\n",
                                    )
                                    .with_placeholder(&available_voiceid_specified_engine)
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .with_help_message(
                                        "Click here https://tinyurl.com/3wzknfnw to learn more",
                                    )
                                    .prompt()
                                    .unwrap();
                                    let mut vec_of_lang_codes = Vec::new();
                                    lang_codes.into_iter().for_each(|lang_code| {
                                        if let Some(langcode) = lang_code {
                                            vec_of_lang_codes.push(langcode.as_str().to_string());
                                        }
                                    });
                                    let available_langcodes_specified_engine = format!("Language codes for the specified engine: {engine_name}\n{:?}\n",vec_of_lang_codes.join(" | "));

                                    let language_code = Text::new("Select the audio language\n")
                                        .with_placeholder(&available_langcodes_specified_engine)
                                        .with_formatter(&|str| format!(".....{str}.....\n"))
                                        .with_help_message(
                                            "Click here https://tinyurl.com/27f3zbhd to learn more",
                                        )
                                        .prompt()
                                        .unwrap();

                                    let possible_text_types = "ssml | text";
                                    let text_type = Text::new("Please provide the text format of the content for which you would like to synthesize audio\n")
                         .with_placeholder(possible_text_types)
                         .with_formatter(&|str| format!(".....{str}.....\n"))
                         .with_help_message("Click here https://tinyurl.com/zyuwuvhp to learn more")
                         .prompt()
                         .unwrap();
                                    let text_to_generate_speech = Text::new("Please upload the text file for which you'd like to generate audio\n")
                         .with_placeholder("The format of the text content is determined by the preceding selections\n")
                         .with_help_message("Click here https://tinyurl.com/ynjmpur3 to Learn more")
                         .with_formatter(&|str| format!(".....{str}.....\n"))
                         .prompt()
                         .unwrap();
                                    let valid_formats = "json | mp3 | ogg_vorbis | pcm";
                                    let audio_output_format = Text::new("Please select the output format for the generated speech content\n")
                        .with_placeholder(valid_formats)
                        .with_formatter(&|str| format!(".....{str}.....\n"))
                        .prompt()
                        .unwrap();
                                    let available_buckets = format!(
                                        "Available Buckets in your account: {:#?}\n",
                                        s3_ops.get_buckets().await
                                    );
                                    let bucket_name = Text::new("Amazon S3 bucket name to which the output file will be saved\n")
                         .with_placeholder(&available_buckets)
                         .with_formatter(&|str| format!(".....{str}.....\n"))
                         .with_help_message("The chosen bucket name should be available in different regions to enable access")
                         .prompt()
                         .unwrap();
                                    match (
                                        voice_id.is_empty(),
                                        language_code.is_empty(),
                                        text_type.is_empty(),
                                        text_to_generate_speech.is_empty(),
                                        audio_output_format.is_empty(),
                                        bucket_name.is_empty(),
                                    ) {
                                        (false, false, false, false, false, false) => {
                                            let synthesise_info = polly_ops
                                                .start_synthesise_task(
                                                    &engine_name,
                                                    &voice_id,
                                                    &language_code,
                                                    &text_type,
                                                    &text_to_generate_speech,
                                                    &audio_output_format,
                                                    &bucket_name,
                                                )
                                                .await;
                                            let status = synthesise_info.get_task_status();
                                            let engine = synthesise_info.get_engine();
                                            let output_uri = synthesise_info.get_output_uri();
                                            let output_format = synthesise_info.get_output_format();
                                            let text_type = synthesise_info.get_text_type();
                                            let voice_id = synthesise_info.get_voice_id();
                                            let language_code = synthesise_info.get_language_code();

                                            if let (
                                                Some(status),
                                                Some(engine),
                                                Some(uri),
                                                Some(format),
                                                Some(text),
                                                Some(voice),
                                                Some(code),
                                            ) = (
                                                status,
                                                engine,
                                                output_uri,
                                                output_format,
                                                text_type,
                                                voice_id,
                                                language_code,
                                            ) {
                                                let colored_status = status.green().bold();
                                                let colored_engine = engine.green().bold();
                                                let colored_uri = uri.green().bold();
                                                let colored_format = format.green().bold();
                                                let colored_type = text.green().bold();
                                                let colored_voiceid = voice.green().bold();
                                                let colored_code = code.green().bold();
                                                println!("This information is obtained from the AWS REST API\n");
                                                println!("Task Status: {colored_status}\n");
                                                println!("Engine Name: {colored_engine}\n");
                                                println!("Output Format of the synthesized audio: {colored_format}\n");
                                                println!("Voice ID of the synthesized audio: {colored_voiceid}\n");
                                                println!("Text type of synthesized audio: {colored_type}\n");
                                                println!("Language Code for the synthesized audio: {colored_code}\n");
                                                println!("The URL for the audio will remain valid for up to 72 hours, which is equivalent to 3 days\n");
                                                println!("URL for the synthesized audio: {colored_uri}\n");
                                                let mut file = OpenOptions::new()
                                                    .create(true)
                                                    .read(true)
                                                    .write(true)
                                                    .open("audio_uri.txt")
                                                    .unwrap();
                                                let uri_data = format!("URL for the synthesized audio: {colored_uri}\n");

                                                file.write_all(uri_data.as_bytes())
                                                    .expect("Error while writing...");
                                                println!(
                                                    "{}\n",
                                                    "URL is writen to current directory"
                                                        .green()
                                                        .bold()
                                                );
                                            }
                                        }
                                        _ => println!(
                                            "{}\n",
                                            "Fields can't be left empty".red().bold()
                                        ),
                                    }
                                }
                                true => {
                                    println!("{}\n", "Engine name can't be left empty".red().bold())
                                }
                            }
                        }

                        "List Speech Synthesis Tasks\n" => {
                            polly_ops.list_synthesise_speech().await;
                        }

                        "Retrieve voice information from Amazon Polly\n" => {
                            let info = polly_ops.describe_voices().await;
                            info.iter()
                .take(3)
                .for_each(|voice_info|{
              
                 if let (Some(gender),Some(voiceid),Some(lang_code),Some(lang_name),Some(voice_name),Some(engines)) = 
                  (voice_info.get_gender(),voice_info.get_voiceid(),voice_info.get_language_code(),
                 voice_info.get_language_name(),voice_info.get_voice_name(),voice_info.get_supported_engines())
                  {
                     println!("Gender: {}\nVoiceId: {}\nLanguageCode: {}\nLanguage Name: {}\nVoice Name: {}",
                     gender.green().bold(),
                     voiceid.green().bold(),
                     lang_code.green().bold(),
                     lang_name.green().bold(),
                     voice_name.green().bold()
                    );
                    engines.iter()
                    .for_each(|engine|{
                     println!("Supported Engine: {}\n",engine.green().bold());
                    });
                 }
                });

                            let mut file = OpenOptions::new()
                                .create(true)
                                .read(true)
                                .write(true)
                                .open("voices_info.txt")
                                .unwrap();
                            let colored_file_name = "'voices_info.txt'".green().bold();
                            let msg = format!("There is a lot more information available, so it only displays the first three pieces of voice information.\n\nAll the voice information is saved to the current directory as {colored_file_name} instead of cluttering the command-line window");
                            println!("{}\n", msg);
                            info.into_iter()
                .for_each(|voice_info|{
                 if let (Some(gender),Some(voiceid),Some(lang_code),Some(lang_name),Some(voice_name),Some(engines)) = 
                  (voice_info.get_gender(),voice_info.get_voiceid(),voice_info.get_language_code(),
                 voice_info.get_language_name(),voice_info.get_voice_name(),voice_info.get_supported_engines())
                  {
                     let data = format!("Gender:           {}\nVoiceId:          {}\nLanguageCode:     {}\nLanguage Name:    {}\nVoice Name:       {}\nSupported Engine: {}\n\n",
                     gender,
                     voiceid,
                     lang_code,
                     lang_name,
                     voice_name,
                     engines.into_iter().collect::<String>()
                 );
                 
                  file.write_all(data.as_bytes())
                  .expect("Error while writing data...")
                 }
                });

                            println!(
                                "{}\n",
                                "Content is writen to current directory".green().bold()
                            );
                        }

                        "Go To Main Menu\n" => continue 'main,

                        _ => println!("Never Reach"),
                    }
                }
            }
            "Amazon Rekognition Operations\n" => {
                let rekog_ops = vec![
                    "Face detection\n",
                    "Text detection\n",
                    "Start a face detection task\n",
                    "Get face detection results\n",
                    "Start a text detection task\n",
                    "Get text detection results\n",
                    "Create Face Liveness task\n",
                    "Get face liveness results\n",
                    "Go to the main menu\n",
                ];
                loop {
                    let rekog_choices = Select::new(
                        "Select the option to execute the operation\n",
                        rekog_ops.clone(),
                    )
                    .with_page_size(9)
                    .prompt()
                    .unwrap();
                    match rekog_choices {
                        "Face detection\n" => {
                            let get_buckets = s3_ops.get_buckets().await;
                            let available_buckets =
                                format!("Available buckets in your account: {:#?}\n", get_buckets);
                            let blob = "https://docs.rs/aws-sdk-rekognition/latest/aws_sdk_rekognition/primitives/struct.Blob.html";
                            let help_message = format!("S3 buckets are employed instead of {blob} types for processing face images");
                            let bucket_name = Text::new(
                                "Select the bucket name where the face image is stored\n",
                            )
                            .with_placeholder(&available_buckets)
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .with_help_message(&help_message)
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
                                    let object = Text::new("Please input the key or path of the face image within the chosen bucket or copy it from the placeholder information\n")
                                        .with_placeholder(&available_objects)
                                        .with_formatter(&|str| format!(".....{str}.....\n"))
                                        .with_help_message("Don't put quotation marks around the key when pasting")
                                        .prompt()
                                        .unwrap();
                                    match object.is_empty() {
                                        false => {
                                            let face_info = rekognition_ops
                                                .detect_faces(&object, &bucket_name)
                                                .await;
                                            face_info.into_iter().for_each(|facedetails| {
                                                let gender = facedetails.get_gender();
                                                let age = facedetails.age_range();
                                                let smile = facedetails.get_smile();
                                                let beard = facedetails.get_beard();

                                                if let (
                                                    (Some(gender), Some(gender_confidence)),
                                                    (Some(age), Some(age_confidence)),
                                                    (Some(smile), Some(smile_confidence)),
                                                    (Some(beard), Some(beard_confidence)),
                                                ) = (gender, age, smile, beard)
                                                {
                                                    println!(
                                                        "Gender: {} and Confidence Level: {}\n",
                                                        gender.green().bold(),
                                                        gender_confidence
                                                            .to_string()
                                                            .green()
                                                            .bold()
                                                    );
                                                    println!(
                                                        "Age: {} and Confidence Level: {}\n",
                                                        age.to_string().green().bold(),
                                                        age_confidence.to_string().green().bold()
                                                    );
                                                    println!(
                                                        "Beard: {} and Confidence Level: {}\n",
                                                        beard.to_string().green().bold(),
                                                        beard_confidence.to_string().green().bold()
                                                    );
                                                    println!(
                                                        "Smile: {} and Confidence Level: {}\n",
                                                        smile.to_string().green().bold(),
                                                        smile_confidence.to_string().green().bold()
                                                    );
                                                }
                                            });
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
                        "Text detection\n" => {
                            let get_buckets = s3_ops.get_buckets().await;
                            let available_buckets =
                                format!("Available buckets in your account: {:#?}\n", get_buckets);
                            let blob = "https://docs.rs/aws-sdk-rekognition/latest/aws_sdk_rekognition/primitives/struct.Blob.html";
                            let help_message = format!("S3 buckets are employed instead of {blob} types for processing texts");
                            let bucket_name = Text::new(
                                "Select the bucket name where the text video is stored\n",
                            )
                            .with_placeholder(&available_buckets)
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .with_help_message(&help_message)
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
                                    let object = Text::new("Please input the key or path of the text video within the chosen bucket or copy it from the placeholder information\n")
                                        .with_placeholder(&available_objects)
                                        .with_formatter(&|str| format!(".....{str}.....\n"))
                                        .with_help_message("Don't put quotation marks around the key when pasting")
                                        .prompt()
                                        .unwrap();
                                    match object.is_empty() {
                                        false => {
                                            let text_info = rekognition_ops
                                                .detect_texts(&bucket_name, &object)
                                                .await;
                                            text_info.into_iter().for_each(|textdetails| {
                                                let texts = textdetails.get_detected_text();
                                                let text_type = textdetails.get_text_type();
                                                let confidence = textdetails.get_confidence();

                                                if let (
                                                    Some(text),
                                                    Some(text_type),
                                                    Some(confidence),
                                                ) = (texts, text_type, confidence)
                                                {
                                                    println!(
                                                        "Detected Text: {}\n",
                                                        text.green().bold(),
                                                    );
                                                    println!(
                                                        "Text Type: {}\n",
                                                        text_type.green().bold(),
                                                    );
                                                    println!(
                                                        "Confidence Level: {}\n",
                                                        confidence.to_string().green().bold(),
                                                    );
                                                }
                                            });
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
                        "Start a face detection task\n" => {
                            let get_buckets = s3_ops.get_buckets().await;
                            let available_buckets =
                                format!("Available buckets in your account: {:#?}\n", get_buckets);

                            let help_message =
                                format!("S3 buckets are used to store face and videos.");
                            let bucket_name = Text::new(
                                "Select the bucket name where the face video is stored\n",
                            )
                            .with_placeholder(&available_buckets)
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .with_help_message(&help_message)
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
                                    let key_video_name = Text::new("Please input the key or path of the face video within the chosen bucket or copy it from the placeholder information\n")
                                        .with_placeholder(&available_objects)
                                        .with_formatter(&|str| format!(".....{str}.....\n"))
                                        .with_help_message("Don't put quotation marks around the key when pasting")
                                        .prompt()
                                        .unwrap();
                                    match key_video_name.is_empty() {
                                        false => {
                                            rekognition_ops
                                                .start_face_detection_task(
                                                    &bucket_name,
                                                    &key_video_name,
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
                        "Get face detection results\n" => {
                            let job_id = Text::new("To obtain the results of the face detection task, please enter the job ID\n")
                                .with_placeholder("The job ID was generated when you initiated the start face detection task\n")
                                .with_formatter(&|str| format!("......{str}......"))
                                .prompt()
                                .unwrap();

                            match job_id.is_empty() {
                                false => {
                                    let face_info =
                                        rekognition_ops.get_face_detection_results(&job_id).await;
                                    let job_status = face_info.get_job_status();
                                    let status_message = face_info.get_status_message();
                                    let face_detail = face_info.get_face_detection();
                                    if let (Some(job_status), Some(status_msg)) =
                                        (job_status, status_message)
                                    {
                                        println!("Job Status is: {}\n", job_status.green().bold());
                                        println!(
                                            "Status Message is: {}\n",
                                            status_msg.green().bold()
                                        );
                                    }
                                    face_detail.into_iter().for_each(|face_detection| {
                                        let timestamp = face_detection.timestamp();
                                        let face = face_detection.face();
                                        if let Some(face_details) = face {
                                            let facedetails =
                                                FaceDetails::build(face_details.to_owned());
                                            let beard = facedetails.get_beard();
                                            let smile = facedetails.get_smile();
                                            let gender = facedetails.get_gender();
                                            let age = facedetails.age_range();
                                            if let (
                                                (Some(gender), Some(gender_confidence)),
                                                (Some(age), Some(age_confidence)),
                                                (Some(smile), Some(smile_confidence)),
                                                (Some(beard), Some(beard_confidence)),
                                            ) = (gender, age, smile, beard)
                                            {
                                                println!(
                                                    "Timestamp: {}\n",
                                                    timestamp.to_string().blue().bold()
                                                );
                                                println!(
                                                    "Gender: {} and Confidence Level: {}\n",
                                                    gender.green().bold(),
                                                    gender_confidence.to_string().green().bold()
                                                );
                                                println!(
                                                    "Age: {} and Confidence Level: {}\n",
                                                    age.to_string().green().bold(),
                                                    age_confidence.to_string().green().bold()
                                                );
                                                println!(
                                                    "Beard: {} and Confidence Level: {}\n",
                                                    beard.to_string().green().bold(),
                                                    beard_confidence.to_string().green().bold()
                                                );
                                                println!(
                                                    "Smile: {} and Confidence Level: {}\n",
                                                    smile.to_string().green().bold(),
                                                    smile_confidence.to_string().green().bold()
                                                );
                                            }
                                        }
                                    });
                                }
                                true => {
                                    println!("{}\n", "Job ID can't be empty".red().bold())
                                }
                            }
                        }
                        "Start a text detection task\n" => {
                            let get_buckets = s3_ops.get_buckets().await;
                            let available_buckets =
                                format!("Available buckets in your account: {:#?}\n", get_buckets);

                            let help_message =
                                format!("S3 buckets are used to store text and videos");
                            let bucket_name = Text::new(
                                "Select the bucket name where the text video is stored\n",
                            )
                            .with_placeholder(&available_buckets)
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .with_help_message(&help_message)
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
                                    let key_text_name = Text::new("Please input the key or path of the text video within the chosen bucket or copy it from the placeholder information\n")
                                        .with_placeholder(&available_objects)
                                        .with_formatter(&|str| format!(".....{str}.....\n"))
                                        .with_help_message("Don't put quotation marks around the key when pasting")
                                        .prompt()
                                        .unwrap();
                                    match key_text_name.is_empty() {
                                        false => {
                                            rekognition_ops
                                                .start_text_detection_task(
                                                    &bucket_name,
                                                    &key_text_name,
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
                        "Get text detection results\n" => {
                            let job_id = Text::new("To obtain the results of the text detection task, please enter the job ID\n")
                            .with_placeholder("The job ID was generated when you initiated the start text detection task\n")
                            .with_formatter(&|str| format!("......{str}......"))
                            .prompt()
                            .unwrap();
                            match job_id.is_empty() {
                                false => {
                                    let text_results =
                                        rekognition_ops.get_text_detection_results(&job_id).await;
                                    let job_status = text_results.get_job_status();
                                    let status_message = text_results.get_status_message();
                                    let text_detection = text_results.get_text_detect_result();
                                    if let (Some(job_status), Some(status_msg)) =
                                        (job_status, status_message)
                                    {
                                        println!("Job Status is: {}\n", job_status.green().bold());
                                        println!(
                                            "Status Message is: {}\n",
                                            status_msg.green().bold()
                                        );
                                    }
                                    text_detection.into_iter().for_each(|text_outputs| {
                                        let timestamp = text_outputs.timestamp();
                                        let get_text = text_outputs.text_detection();
                                        if let Some(text_detection) = get_text {
                                            let textdetails =
                                                TextDetect::build(text_detection.to_owned());

                                            let texts = textdetails.get_detected_text();
                                            let text_type = textdetails.get_text_type();
                                            let confidence = textdetails.get_confidence();
                                            println!(
                                                "Timestamp: {}\n",
                                                timestamp.to_string().green().bold()
                                            );

                                            if let (Some(text), Some(text_type), Some(confidence)) =
                                                (texts, text_type, confidence)
                                            {
                                                println!(
                                                    "Detected Text: {}\n",
                                                    text.green().bold(),
                                                );
                                                println!(
                                                    "Text Type: {}\n",
                                                    text_type.green().bold(),
                                                );
                                                println!(
                                                    "Confidence Level: {}\n",
                                                    confidence.to_string().green().bold(),
                                                );
                                            }
                                        }
                                    });
                                }
                                true => println!("{}\n", "Job ID can't be empty".red().bold()),
                            }
                        }
                        "Create Face Liveness task\n" => {
                            let get_buckets = s3_ops.get_buckets().await;
                            let available_buckets =
                                format!("Available buckets in your account: {:#?}\n", get_buckets);

                            let help_message = format!("S3 buckets are used to store videos");
                            let bucket_name = Text::new(
                                "Select the bucket name where the face data will be stored when using a liveness session\n",
                            )
                            .with_placeholder(&available_buckets)
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .with_help_message(&help_message)
                            .prompt()
                            .unwrap();
                            match bucket_name.is_empty() {
                                false => {
                                    rekognition_ops.create_face_liveness(&bucket_name).await;
                                }
                                true => println!("{}\n", "Bucket name can't be empty".red().bold()),
                            }
                        }
                        "Get face liveness results\n" => {
                            let session_id= Text::new(
                                "Please enter the session ID to retrieve the FaceLiveness results\n",
                            )
                            .with_placeholder("The session ID is generated when you call the CreateFaceLiveness REST API or\nis written to the current directory if you used the 'createfaceliveness' option")
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .with_help_message("")
                            .prompt()
                            .unwrap();
                            match session_id.is_empty() {
                                false => {
                                    let results = rekognition_ops
                                        .get_face_liveness_session_results(&session_id)
                                        .await;
                                    let status = results.get_liveness_status();
                                    let confidence_level = results.get_confidence();
                                    let reference_image_type = results.get_reference_image_type();

                                    if let (Some(status), Some(confidence), Some(ref_type)) =
                                        (status, confidence_level, reference_image_type.flatten())
                                    {
                                        println!(
                                            "Status of FaceLivenessTask: {}\n",
                                            status.green().bold()
                                        );
                                        println!(
                                            "Confidence Level of Image: {}\n",
                                            confidence.to_string().green().bold()
                                        );
                                        let s3_info = ref_type.get_s3_info();
                                        let bbox = ref_type.get_bounding_box_info();
                                        if let (
                                            (Some(bucket_name), Some(bucket_key_name)),
                                            (Some(width), Some(height), Some(left), Some(top)),
                                        ) = (s3_info, bbox)
                                        {
                                            println!("The bucket name where the session is created and the key name where the reference images are stored are as follows\n");
                                            println!(
                                                "Bucket Name: {} and Bucket Key: {}\n",
                                                bucket_name.green().bold(),
                                                bucket_key_name.green().bold()
                                            );
                                            println!("Bounding box details\n");
                                            println!(
                                                "Width: {}\n",
                                                width.to_string().green().bold()
                                            );
                                            println!(
                                                "Height: {}\n",
                                                height.to_string().green().bold()
                                            );
                                            println!("Left: {}\n", left.to_string().green().bold());
                                            println!("Top: {}\n", top.to_string().green().bold());
                                        }
                                    }
                                }
                                true => {
                                    println!("{}\n", "The Session ID can't be empty".red().bold())
                                }
                            }
                        }
                        "Go to the main menu\n" => continue 'main,
                        _ => println!("Never Reach"),
                    }
                }
            }

            "AWS Simple Notification Service(SNS) Operations\n" => {
                let sns_operations = vec![
                    "Create Topic\n",
                    "Subscription Under Topic\n",
                    "Add Phone Number\n",
                    "Verify the pending status of the phone number\n",
                    "Send Messages to Phone Numbers in a Topic\n",
                    "Go To Main Menu\n",
                ];

                loop {
                    let sns_choices = Select::new("Select the option to execute the operation\n", sns_operations.clone())
                        .with_page_size(6)
                        .with_help_message("These options are tailored for SMS services, rather than other notification services")
                        .prompt()
                        .unwrap();
                    match sns_choices {
                        "Create Topic\n" => {
                            let topic_name = Text::new("Enter the topic name\n")
                                .with_placeholder("This topic name also serves as the project name")
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .prompt()
                                .unwrap();
                            match topic_name.is_empty() {
                                false => {
                                    sns_ops.create_topic(&topic_name).await;
                                }
                                true => {
                                    println!("{}\n", "Topic Name can't be left empty".red().bold())
                                }
                            }
                        }
                        "Subscription Under Topic\n" => {
                            let topic_arn = Text::new("Enter the topic ARN to subscribe to\n")
                                .with_placeholder("The topic ARN is generated and written to the current directory if you used the previous option; otherwise, go to the SNS topic page to obtain the ARN\n")
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .prompt()
                                .unwrap();
                            let some_possible_protocols ="Some possible Values\n'http' - delivery of JSON-encoded message via HTTP POST\n'email' - delivery of message via SMTP\n'sms' -delivery of message via SMS";
                            let protocol = Text::new("Please specify the protoco\n")
                                .with_placeholder(some_possible_protocols)
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .with_help_message(
                                    "Click here https://tinyurl.com/2dkwfdpn to learn more",
                                )
                                .prompt()
                                .unwrap();
                            let end_point = Text::new("Please specify the endpoint\n")
                            .with_placeholder("The endpoint depends on the protocol you selected in the previous option\n")
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .with_help_message("Click here https://tinyurl.com/2dkwfdpn to learn more")
                            .prompt()
                            .unwrap();
                            match (
                                topic_arn.is_empty(),
                                protocol.is_empty(),
                                end_point.is_empty(),
                            ) {
                                (false, false, false) => {
                                    sns_ops
                                        .subscription(&topic_arn, &protocol, &end_point)
                                        .await;
                                }
                                _ => println!("{}\n", "No Fields can't be left empty".red().bold()),
                            }
                        }
                        "Add Phone Number\n" => {
                            let phone_number = Text::new("Please provide the phone number\n")
                                .with_placeholder(
                                    "Ensuring it includes the country code before the digits\n",
                                )
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .prompt()
                                .unwrap();
                            match phone_number.is_empty() {
                                false => {
                                    sns_ops.create_sandbox_phone_number(&phone_number).await;
                                    let confirm_to_verify = Confirm::new(
                                        "Do you want to verify the phone number as well?\n",
                                    )
                                    .with_placeholder("Yes, to receive an OTP.No, just to add it")
                                    .prompt()
                                    .unwrap();
                                    match confirm_to_verify {
                                        true => {
                                            let otp = Text::new(
                                                "Please enter the OTP sent to your mobile",
                                            )
                                            .with_placeholder("It consists of 6 digits")
                                            .with_formatter(&|str| format!(".....{str}.....\n"))
                                            .prompt()
                                            .unwrap();
                                            match otp.is_empty() {
                                                false => {
                                                    sns_ops
                                                        .verify_phone_number(&phone_number, &otp)
                                                        .await;
                                                    dotenv().ok();
                                                    let topic_arn = var("TOPIC_ARN").unwrap();
                                                    sns_ops
                                                        .subscription(
                                                            &topic_arn,
                                                            "sms",
                                                            &phone_number,
                                                        )
                                                        .await;
                                                }
                                                true => {
                                                    println!(
                                                        "{}\n",
                                                        "Otp can't be empty".red().bold()
                                                    );
                                                    continue;
                                                }
                                            }
                                        }
                                        false => println!("{}\n", "Sure..".green().bold()),
                                    }
                                }
                                true => {
                                    println!("{}\n", "Phone Number can't be empty".red().bold())
                                }
                            }
                        }
                        "Verify the pending status of the phone number\n" => {
                            let get_numbers = sns_ops.list_sms_sandbox_numbers().await;
                            let phone_number = Text::new("Please enter the phone number for verification or copy it from the placeholder information\n")
                            .with_placeholder(&get_numbers)
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .with_help_message("Select the phone number with a pending status; otherwise, an error will occur")
                            .prompt()
                            .unwrap();
                            match phone_number.is_empty() {
                                false => {
                                    sns_ops.create_sandbox_phone_number(&phone_number).await;
                                    let info =
                                        format!("Please enter the OTP sent to: {phone_number}\n");
                                    let otp = Text::new(&info)
                                        .with_placeholder("It consists of 6 digits")
                                        .with_formatter(&|str| format!(".....{str}.....\n"))
                                        .prompt()
                                        .unwrap();
                                    match otp.is_empty() {
                                        false => {
                                            sns_ops.verify_phone_number(&phone_number, &otp).await;
                                            dotenv().ok();
                                            let topic_arn = var("TOPIC_ARN").unwrap();
                                            sns_ops
                                                .subscription(&topic_arn, "sms", &phone_number)
                                                .await;
                                        }
                                        true => {
                                            println!("{}\n", "Otp can't be empty".red().bold());
                                            continue;
                                        }
                                    }
                                }
                                true => {
                                    println!("{}\n", "Phone Number can't be empty".red().bold())
                                }
                            }
                        }
                        "Send Messages to Phone Numbers in a Topic\n" => {
                            let topic_arn = Text::new("Enter the topic ARN to send messages to\n")
                                .with_placeholder("The ARN is generated when you create a topic\n")
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .with_help_message("If you used the 'create topic' option to create the topic, then the ARN is stored in the current directory")
                                .prompt()
                                .unwrap();

                            let message = Text::new("Enter the message you want to send\n")
                            .with_placeholder("This data will be sent to all the subscribers in the given topic ARN\n")
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .prompt()
                            .unwrap();
                            match (topic_arn.is_empty(), message.is_empty()) {
                                (false, false) => {
                                    sns_ops.publish(&message, &topic_arn).await;
                                }
                                _ => println!("{}\n", "No fields can't be left empty".red().bold()),
                            }
                        }

                        "Go To Main Menu\n" => continue 'main,

                        _ => println!("Never Reach\n"),
                    }
                }
            }

            "AWS Simple Email Service(SES) Operations\n" => {
                let ses_operations = vec![
    "Create a Contact List Name\n",
    "Add an email to the list\n",
    "Default Values\n",
    "Email Verification\n",
    "Print the emails from the provided list\n",
    "Send a Simple Email to a Specific Recipient\n",
    "Send a Templated Email to a Specified Email Address\n",
    "Send a simple email with the same body and subject to all the email addresses in the list\n",
    "Send templated Emails\n",
    "Common Errors\n",
    "Go to main menu\n",
    ];
                loop {
                    let email_choice = Select::new(
                        "Select the option to execute the operation\n",
                        ses_operations.clone(),
                    )
                    .with_help_message("Do not enclose it with quotation marks or add spaces")
                    .with_vim_mode(true)
                    .with_page_size(11)
                    .prompt()
                    .unwrap();

                    match email_choice {
            "Create a Contact List Name\n" => 
            
            {
                let lst_name = Text::new("Enter the list name to add to the AWS Simple Email Service\n")
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
                        ses_ops.create_contact_list_name(&lst_name, Some(description)).await;
                    }
                    (false,true) =>{
                        ses_ops.create_contact_list_name(&lst_name, None).await;
                    },
                    _ => println!("{}\n","Contact Name Can't be empty..try again".red().bold()),
                }
            }

            "Add an email to the list\n" => {
                let get_contact_list_name = ses_ops.get_list_name();
                let get_contact_list_name = format!("Default contact list name: {}\n",get_contact_list_name);          
                let  email = Text::new("Enter the email\n")
                    .with_placeholder("Emails should be without quotation marks around them\n")
                    .with_formatter(&|str| format!(".....{str}.....\n"))
                    .prompt()
                    .unwrap();
                let list_name = Text::new("Enter the list name you want the email add in it\n")
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
                                          
                                
                match (list_name.is_empty(),email.is_empty(),to_verified) {
                    (false,false,false) => {
                        println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                        ses_ops.create_email_contact_without_verification(&email, Some(&list_name)).await;
                    },
                    (false,false,true) =>{
                        println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                        ses_ops.create_email_contact_with_verification(&email,Some(&list_name)).await;
                    }
                   (true,false,false) =>{
                      println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                       ses_ops.create_email_contact_without_verification(&email,None).await;
                   }
                   (true,false,true) =>{
                    println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                      ses_ops.create_email_contact_with_verification(&email,None).await;
                   }
                    _ => println!("{}\n","No email is received".red().bold()),
                }
            }

            "Email Verification\n" =>{
                let email_to_verify = Text::new("Enter the email to check the identity\n")
                                    .with_placeholder("Only verified email can receive email\n")
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt()
                                    .unwrap();
                        match email_to_verify.is_empty(){
                            false => {
                                println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                               match ses_ops
                                    .is_email_verfied(&email_to_verify)
                                     .await{
                                      true =>{
                                        let email_to_verify = email_to_verify.green().bold();
                                        println!("The email address {email_to_verify} has been verified\n");
                                        println!(" You can use it to receive messages or as a 'from' address\n");
                                    },
                                      false => {
                                        let email_to_verify = email_to_verify.green().bold();
                                        println!("The email address {email_to_verify} is not verified\n");
                                        println!("Therefore, you can't use it to send emails ('from' address) or receive messages\n");
                                      }
                                }

                            }
                            true =>{
                               println!("{}\n","The email can't be empty".red().bold())
                            }

                        }

            }


            "Print the emails from the provided list\n" => {
              
                let get_contact_list_name=ses_ops.get_list_name();
                let get_contact_list_name = format!("Default contact list name: {}\n",get_contact_list_name);
                

                  let list_name = Text::new("Please enter the name of the list for which you'd like to print emails\n")
                                       .with_placeholder(&get_contact_list_name)
                                       .with_formatter(&|str| format!(".....{str}....."))
                                       .prompt_skippable()
                                       .unwrap()
                                       .unwrap();
                 let print_emails = Confirm::new("You are tasked with printing all the emails in the list\n")
                                       .with_placeholder("Select 'Yes' to print emails or 'No' to save them to the current directory\n")
                                       .with_formatter(&|str| format!(".....{str}....."))
                                       .prompt_skippable()
                                       .unwrap()
                                       .unwrap();
                 match list_name.is_empty() {
                     false =>{
                           match print_emails{
                            true =>{
                                let upto = Text::new("How many emails would you like to print?\n")
                                                     .with_placeholder("Values should be non-zero; if no value is provided, it defaults to zero\n")
                                                     .with_formatter(&|str| format!(".....{str}....."))
                                                     .prompt_skippable()
                                                     .unwrap()
                                                     .unwrap();
                                    match upto.is_empty(){
                                        false =>{
                                            println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                                            let parse_to_digit = upto.parse::<usize>().unwrap();
                                            ses_ops.printing_email_addresses_from_provided_list(Some(&list_name), true, Some(parse_to_digit)).await;

                                        }
                                        true =>{
                                            println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                                            ses_ops.printing_email_addresses_from_provided_list(Some(&list_name), true, None).await;

                                        }

                                    }

                            },
                            false =>{
                            ses_ops.printing_email_addresses_from_provided_list(Some(&list_name), false, None).await;
  
                            }
                           }
                     }
                     true => {
                        match print_emails{
                            true =>{
                                let upto = Text::new("How many emails would you like to print?\n")
                                                     .with_placeholder("Values should be non-zero; if no value is provided, it defaults to zero\n")
                                                    .with_formatter(&|str| format!(".....{str}....."))
                                                     .prompt_skippable()
                                                     .unwrap()
                                                     .unwrap();
                                    match upto.is_empty(){
                                        false =>{
                                            println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                                            let parse_to_digit = upto.parse::<usize>().unwrap();
                                            ses_ops.printing_email_addresses_from_provided_list(None, true, Some(parse_to_digit)).await;

                                        }
                                        true =>{
                                            println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());  
                                            ses_ops.printing_email_addresses_from_provided_list(None, true, None).await;

                                        }

                                    }

                            },
                            false =>{
                                println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                                ses_ops.printing_email_addresses_from_provided_list(None, false, None).await;  
                            }
                         

                     }
                 }                   
                    
 }},
            "Default Values\n" => {
                let default_list_name = ses_ops.get_list_name().green().bold();
                let default_template_name = ses_ops.get_template_name().green().bold();
                let default_from_address = ses_ops.get_from_address().green().bold();
                println!("Default Contact List Name: {default_list_name}\n");
                println!("Default Template Name: {default_template_name}\n");
                println!("Default from_address is: {default_from_address}\n");

                println!("{}\n","Be sure to check the placeholder for default values, allowing you to skip using the default value".bright_blue());
            }


            "Send a Simple Email to a Specific Recipient\n" => {

                let email = Text::new("Enter the email..\n")
                                   .with_formatter(&|str| format!(".....{str}....."))
                                   .prompt()
                                   .unwrap();
                println!("{email}");

                let subject = Text::new("Enter the subject of Email\n")
                                 .with_formatter(&|str| format!(".....{str}....."))
                                 .prompt().unwrap();

                let body = Text::new("Enter the message body to display to the recipient\n")
                                 .with_formatter(&|str| format!(".....{str}....."))
                                .prompt()
                                .unwrap();
                 
                let defaul_from_address = ses_ops.get_from_address();
        
                let default_from_address =format!("Your 'from_address' needs to be verified, which is typically your email\nand the default 'from_address' is {}",defaul_from_address);

                let from_address = Text::new("Enter the 'from' address or press enter to use default from_address if any\n")
                    .with_placeholder(&default_from_address)
                    .with_formatter(&|str| format!(".....{str}....."))
                    .prompt_skippable()
                    .unwrap()
                    .unwrap();
                let simple_email = SimpleMail::builder(
                    &body,
                    &subject
                )
                .build();

                match (email.is_empty(), subject.is_empty(), body.is_empty(),from_address.is_empty()) {
                    (false, false, false,false) => {
                             ses_ops
                            .send_mono_email(&email, Simple_(simple_email),Some(&from_address))
                            .await
                            .send()
                            .await
                            .map(|_|{
                                let colored_email = email.green().bold();
                                println!("An Email is send succesfully to: {}",colored_email)
                              })
                            .unwrap();
                          
                    }
                    (false,false,false,true) => {
                          
                                 ses_ops
                                .send_mono_email(&email, Simple_(simple_email),None)
                                .await
                                .send()
                                .await
                                .map(|_|{
                                    let colored_email = email.green().bold();
                                    println!("An Email is send succesfully to: {}",colored_email)
                                   })
                                .unwrap();
                          
                    }
                    _ => {
                        println!("{}\n","Email,subject or body can't be empty".red().bold());
                    }
                }
            }
            "Send a Templated Email to a Specified Email Address\n" => {

                let get_from_address = ses_ops.get_from_address();
                let get_template_name = ses_ops.get_template_name();
                let default_template_name = format!("Default template name is: {}",get_template_name);
                let default_from_address = format!("Default from_address is: {}",get_from_address);
                let email = Text::new("Enter the email you wish to send\n")
                    .with_placeholder("The email must be verified")
                    .with_formatter(&|str| format!(".....{str}....."))
                    .prompt()
                    .unwrap();
    
        let template_name = Text::new(
                "Please enter the template name you want to use for the email\n",)
            .with_placeholder(&default_template_name)
            .with_formatter(&|str| format!(".....{str}.....\n"))
            .with_help_message(
                "The template name must exist, and the variables should be specified as key-value pairs according to the template\n",
            )
            .prompt_skippable()
            .unwrap()
            .unwrap();
            
            let from_address =Text::new("Enter the from address\n")
                .with_placeholder(&default_from_address)
                .with_formatter(&|str| format!(".....{str}.....\n"))
                .prompt_skippable()
                .unwrap()
                .unwrap();
            let template_data = Text::new("Enter the template data\n")
                      .with_formatter(&|str| format!(".....{str}.....\n"))
                      .prompt()
                      .unwrap();
                 
                match (email.is_empty(), template_name.is_empty(),from_address.is_empty()) {
                    //If both email and template is specified we can use those
                    (false, false,false) => {
                        let email_content=TemplateMail::builder(&template_name, &template_data)
                        .build();
                         ses_ops
                        .send_mono_email(&email, Template_(email_content),Some(&from_address))
                        .await
                        .send()
                        .await
                        .map(|_|{
                            let colored_email = email.green().bold();
                            println!("The template email is send to: {}\n",colored_email)
                    })
                        .unwrap();
                    }
                    //If template name is skipped then default template name is used
                    //which might results in error if no template name is None or not exist
                    (false,false,true) => {
                        let email_content=TemplateMail::builder(&template_name, &template_data)
                        .build();
                            ses_ops
                            .send_mono_email(&email, Template_(email_content),None)
                            .await
                            .send()
                            .await
                            .map(|_|{
                                let colored_email = email.green().bold();
                                println!("The template email is send to: {}\n",colored_email)
                                 })
                            .unwrap();
                    }
                
                    (false,true,true) =>{
                        println!("Template data is:\n{}",template_data);
                        let email_content=TemplateMail::builder(&get_template_name, &template_data)
                        .build();
                            ses_ops
                            .send_mono_email(&email, Template_(email_content),None)
                            .await
                            .send()
                            .await
                            .map(|_|{
                                let colored_email = email.green().bold();
                                println!("The template email is send to: {}\n",colored_email)})
                            .unwrap();
                    }
                    (false,true,false) =>{
                        let email_content=TemplateMail::builder(&get_template_name, &template_data)
                        .build();
                            ses_ops
                            .send_mono_email(&email, Template_(email_content),Some(&from_address))
                            .await
                            .send()
                            .await
                            .map(|_|{
                                let colored_email = email.green().bold();
                                println!("The template email is send to: {}\n",colored_email)})
                            .unwrap();
                    }
                    _ => {
                        println!("{}\n","Please ensure the email field is not empty, and try again".red().bold());                      
                    }
                }
            }
            "Send a simple email with the same body and subject to all the email addresses in the list\n" => {
                 
                let get_from_address = ses_ops.get_from_address();
                let get_list_name=ses_ops.get_list_name();

                let default_from_address = format!("Default from_address is: {}\n",get_from_address);
                let default_list_name = format!("Default list name is: {}\n",get_list_name);   

                let list_name = Text::new("Enter the list name..\n")
                    .with_placeholder(&default_list_name)
                    .with_formatter(&|str| format!(".....{str}.....\n"))
                    .prompt_skippable()
                    .unwrap()
                    .unwrap_or(ses_ops.get_list_name().into());
                let body = Text::new("Body details\n")
                    .with_placeholder("The body data is the same for all emails..\n")
                    .with_formatter(&|str| format!(".....{str}.....\n"))
                    .prompt()
                    .unwrap();
                let subject = Text::new("Enter the subject name\n")
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
                let simple_data = SimpleMail::builder(&body, &subject);
                match (subject.is_empty(),body.is_empty()){
                    (false,false)=>{
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
                    _ => println!("{}\n","Both body and subject can't be empty".red().bold()),
                }
            }

            "Send templated Emails\n" => {
                let get_from_address = ses_ops.get_from_address();
                let get_template_name = ses_ops.get_template_name();
                let get_list_name=ses_ops.get_list_name();

                println!("{}\n","This information is for debugging".green().bold());

                let template_name = format!("Template name is: {}\n",get_template_name);
                let list_name = format!("List name is: {}\n",get_list_name);
                let from_address = format!("from address is: {}\n",get_from_address);


                println!("{}\n{}\n{}",template_name,list_name,from_address);
                  ses_ops
                  .send_emails()
                  .await;
            }
         "Common Errors\n" => {
            let possible_errors = include_str!("./possible_errors.txt").blue().italic().bold();
             println!("{}\n",possible_errors);

            let decision = Confirm::new("Go to main or exist")
               .with_placeholder("Yes to main menu , No to Quit")
              .prompt()
              .unwrap();
            if decision{
                continue 'main;
            }
            else{
                break 'main;
            }

         }
         "Go to main menu\n" => continue 'main,

            _ => {}
        }
                }
            }

            "S3 Bucket Operations\n" => {
                let s3_operations = vec![
                    "Create Bucket\n",
                    "Default Region Name\n",
                    "Put object in a Bucket\n",
                    "List objects from a Bucket\n",
                    "Download object from bucket\n",
                    "Retrieve a presigned URL for an object\n",
                    "Get Bucket Lists\n",
                    "Delete object from a bucket\n",
                    "Delete Bucket\n",
                    "Go to main\n",
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
                            println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
                            let bucket_lists = s3_ops.get_buckets().await;
                            for bucket in bucket_lists {
                                println!("    {}\n", bucket.green().bold());
                            }
                        }

                        "List objects from a Bucket\n" => {
                            let get_bucket_name = s3_ops.get_buckets().await;
                            let bucket_names =
                                format!("Available buckets are: {:#?}\n", get_bucket_name);

                            let bucket_name = Text::new("Please input the name of the bucket")
                                .with_placeholder(&bucket_names)
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .prompt()
                                .unwrap();

                            match bucket_name.is_empty() {
                                false => {
                                    println!("{}\n","Data is retrieved from the internet, a process that takes seconds.".blue().bold());
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
                                "These buckets are already in your account: {:#?}",
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
                                        "The object names are in the {bucket_name} bucket: {:#?}\n",
                                        object_names
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
                                "Below, you'll find the buckets in your account: {:?}\n",
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
                                    "You can copy the path and ctrl+shift+v to paste it here",
                                )
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .prompt()
                                .unwrap();

                            let get_bucket_name = s3_ops.get_buckets().await;
                            let available_bucket_name = format!(
                                "Available bucket names in your account: {:?}",
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
                                    s3_ops
                                        .upload_content_to_a_bucket(&bucket_name, &object, &key)
                                        .await;
                                }

                                _ => {
                                    println!("{}\n", "Data ,the key/object name or the bucket name can't be empty".red().bold())
                                }
                            }
                        }

                        "Download object from bucket\n" => {
                            let get_buckets = s3_ops.get_buckets().await;
                            let available_buckets =
                                format!("Available buckets in your account: {:#?}\n", get_buckets);

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
                                                .download_content_from_bcuket(&bucket_name, &object)
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
                            let available_bucket_name =
                                format!("Available buckets in your account: {:?}", get_bucket_name);
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
                        "Go to main\n" => break 's3_ops,
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
                    "Retrieving Connection URL Information\n",
                    "Start Db Instance\n",
                    "Stop Db Instance\n",
                    "Modify Master Password of Database Instance\n",
                    "Delete Db Instance\n",
                    "Describe Db Cluster\n",
                    "Delete Db Cluster\n",
                    "Go to main menu\n",
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
                               .with_placeholder("The DB instance identifier is case-insensitive, but is stored as all lowercase (as in \"mydbinstance\").\nConstraints: 1 to 60 alphanumeric characters or hyphens. First character must be a letter. Can't contain two consecutive hyphens. Can't end with a hyphen\n")
                               .with_formatter(&|str| format!(".....{str}.....\n"))
                               .prompt()
                               .unwrap();
                            let engine =
                                Text::new("Select the database engine for your database system\n")
                                    .with_placeholder(
                                        "Some possible values are: 'mariadb', 'mysql', 'postgres'",
                                    )
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .with_help_message(
                                        "look here to know more http://tinyurl.com/4h8fcwf6",
                                    )
                                    .prompt()
                                    .unwrap();
                            let db_name = Text::new("Select the db name for your database\n")
                                .with_placeholder(
                                    "Some possible values are: 'MySQL', 'MariaDB', 'PostgreSQL'",
                                )
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .with_help_message(
                                    "look here to know more http://tinyurl.com/4mnhdpkm",
                                )
                                .prompt()
                                .unwrap();
                            let storage_type= Text::new("Select the storage type for your database")  
                     .with_placeholder("The storage type and the next database instance class should be a correct combination for successfully creating a database instance\n")
                     .with_formatter(&|str| format!(".....{str}.....\n"))   
                     .with_help_message("Click here http://tinyurl.com/4h8fcwf6 to learn more") 
                     .prompt()
                     .unwrap();
                            let db_instance_class =  Text::new("Select instance class for your database\n")  
            .with_placeholder("The instance class and the previous storage type should be a correct combination for successfully creating a database instance\n")
            .with_formatter(&|str| format!(".....{str}.....\n"))   
            .with_help_message("Click here http://tinyurl.com/29am8kup to learn more") 
            .prompt()
            .unwrap();

                            let allocated_storage = Text::new("Specify the storage capacity for your database in gigabytes, using numerical digits\n")  
            .with_placeholder("The storage requirements depend on your specific use cases and the storage type you have previously selected\n")
            .with_formatter(&|str| format!(".....{str}.....\n"))   
            .with_help_message("Click here http://tinyurl.com/4h8fcwf6 to learn more") 
            .prompt()
            .unwrap();

                            let username = Text::new("Select the username for your database\n")  
            .with_placeholder("The username and password options are required parameters for the database instance\n")
            .with_formatter(&|str| format!(".....{str}.....\n"))  
            .prompt()
            .unwrap();
                            let password = Text::new("Select the password for your database\n")  
            .with_placeholder("Once you have created the database instance, you can obtain the database URL by selecting the 'Get Database URL' option")  
            .with_formatter(&|str| format!(".....{str}.....\n"))
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
                                            let colored_msg ="The choices have been saved to the current directory for your reference\n".green().bold();
                                            println!("{colored_msg}");
                                        }
                                        Err(_) => println!(
                                            "Error while writting file to the current directory\n"
                                        ),
                                    }
                                }
                                _ => println!("{}\n", "Fields cannot be left empty.".red().bold()),
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
                        }

                        "Retrieving Connection URL Information\n" => {
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
                                            let password = Text::new("Enter the password\n")  
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
                                                    rds_ops
                                                        .status_of_db_instance(Some(
                                                            &db_instance_identifier,
                                                        ))
                                                        .await;
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
                                                    println!("Username: {colored_username}\n");
                                                    println!("Endpoint with port: {colored_endpoint_with_port}\n");
                                                    println!("Db Name: {colored_db_name}\n");
                                                    rds_ops
                                                        .status_of_db_instance(Some(
                                                            &db_instance_identifier,
                                                        ))
                                                        .await;
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
                                            let password = Text::new("Enter the password\n")  
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
                                                    rds_ops.status_of_db_instance(None).await;
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
                                                    println!("Username: {colored_username}\n");
                                                    println!("Endpoint with port: {colored_endpoint_with_port}\n");
                                                    println!("Db Name: {colored_db_name}\n");
                                                    rds_ops
                                                        .status_of_db_instance(Some(
                                                            &db_instance_identifier,
                                                        ))
                                                        .await;
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
                            println!("Endpoint_With_Port: {:?}\nZone: {:?}\nInstance class: {:?}\nDb name: {:?}\nStatus of db instance: {:?}\n",
                            endpoint_with_port,zone,class,db_name,status);
                        }

                        "Start Db Instance\n" => {
                            let default_instance_id = format!(
                                "The default instance ID: {} and the current status db instance:\n",
                                rds_ops.get_db_instance_id(),
                            );
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
                            let default_instance_id = format!(
                                "The default instance ID: {}\n",
                                rds_ops.get_db_instance_id()
                            );
                            let db_instance_identifier = Text::new("Enter the database instance identifier for which you want to stop temporarily\n")  
                            .with_placeholder(&default_instance_id)
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .with_help_message("The status of the DB instance should be \"available\"; otherwise, this operation will result in a panic (the Rust way of handling runtime exceptions).")
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
                        "Modify Master Password of Database Instance\n" => {
                            let db_instance_identifier =
                                Text::new("Enter the DB instance ID you wish to modify\n")
                                    .with_placeholder("You can modify only master password\n")
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt()
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
                                        .await
                                }
                                _ => println!("{}\n", "Fields cannot be left empty.".red().bold()),
                            }
                        }

                        "Delete Db Instance\n" => {
                            let default_instance_id = format!(
                                "The default instance ID: {}\n",
                                rds_ops.get_db_instance_id()
                            );
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
                                    rds_ops
                                        .status_of_db_instance(Some(&db_instance_identifier))
                                        .await
                                }
                                true => rds_ops.status_of_db_instance(None).await,
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
                                if let (
                                    Some(status),
                                    Some(cluster_endpoint),
                                    Some(master_username),
                                    Some(db_name),
                                ) = (
                                    status,
                                    cluster_endpoint_with_port,
                                    master_user_name,
                                    cluster_db_name,
                                ) {
                                    let colored_status = status.green().bold();
                                    let colored_endpoint = cluster_endpoint.green().bold();
                                    let colored_username = master_username.green().bold();
                                    let colored_dbname = db_name.green().bold();
                                    println!("Current Status of Cluster: {colored_status}\n");
                                    println!("Cluster endpoint with port: {colored_endpoint}\n");
                                    println!(
                                        "Master Username of the Cluster: {colored_username}\n"
                                    );
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
                        "Go to main menu\n" => continue 'main,

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
                    "Go To Main Menu\n",
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
                                _ => println!("{}\n", "Fields cannot be left empty.".red().bold()),
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
                                _ => println!("{}\n", "Fields cannot be left empty.".red().bold()),
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
                            let available_acl_names = format!("List of Access Control List (ACL) Names in Your Credentials: {:#?}",acl_names);
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
                                    println!("{}\n", "Fields cannot be left empty.".red().bold())
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
                            let available_acl_names = format!("List of Access Control List (ACL) Names in Your Credentials: {:#?}",acl_names);
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
                                _ => println!("{}\n", "Fields cannot be left empty".red().bold()),
                            }
                        }
                        "Go To Main Menu\n" => continue 'main,
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
