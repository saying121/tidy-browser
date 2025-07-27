use std::{collections::HashSet, path::PathBuf, str::FromStr};

use strum::IntoEnumIterator;

use crate::{
    args::{self, ChromiumArgs, ChromiumName, FirefoxArgs, SafariArgs},
    chromium::ChromiumBased,
    error::Result,
};

pub async fn run_cli(args: crate::args::Args) -> Result<()> {
    let output_dir = args
        .output_dir
        .unwrap_or_else(|| PathBuf::from_str("results").unwrap());

    if args.all_browsers {
        ChromiumBased::multi_data(ChromiumName::iter(), output_dir, args.sep).await?;

        return Ok(());
    }
    match args.core {
        args::Core::Chromium(ChromiumArgs { name, user_data_dir, host, values }) => {
            ChromiumBased::write_data(
                name,
                user_data_dir,
                host,
                HashSet::from_iter(values.into_iter()),
                output_dir,
                args.sep,
            )
            .await?;
        },
        args::Core::Firefox(FirefoxArgs {
            name,
            base,
            profile,
            profile_path,
            host,
            values,
        }) => {},
        #[cfg(target_os = "macos")]
        args::Core::Safari(SafariArgs { values }) => {},
    }

    Ok(())
}

// #![expect(warnings)]
//
// use decrypt_cookies::prelude::*;
// use miette::{IntoDiagnostic, Result};
// use tokio::{
//     fs::{create_dir_all, File, OpenOptions},
//     io::{AsyncWriteExt, BufWriter},
// };
//
// const BASE_DIR: &str = "./results";
//
// async fn open_file(browser: &str, item: &str) -> Result<BufWriter<File>> {
//     Ok(BufWriter::new(
//         OpenOptions::new()
//             .create(true)
//             .write(true)
//             .open(format!("{BASE_DIR}/{browser}-{item}.csv"))
//             .await
//             .into_diagnostic()?,
//     ))
// }

// async fn write_chromium_password<T: Sync + Send>(getter: &ChromiumGetter<T>) -> Result<()> {
//     println!("{} password", getter);
//     let getter = ChromiumBuilder::<Chrome>::new()
//         .build()
//         .await?;
//     let all_passwords = getter.get_logins_all().await?;
//     let head = b"Url,Username,Password,CreateDate,LastUsedDate,PasswordModifiedDate\n";
//     let mut buf_writer = open_file(&getter.to_string(), "password").await?;
//     buf_writer
//         .write_all(head)
//         .await
//         .into_diagnostic()?;
//
//     for ck in all_passwords {
//         let pass_str = format!(
//             "{},{},{},{},{},{}\n",
//             ck.origin_url,
//             ck.username_value
//                 .unwrap_or_default(),
//             ck.password_value
//                 .unwrap_or_default(),
//             ck.date_created.unwrap_or_default(),
//             ck.date_last_used
//                 .unwrap_or_default(),
//             ck.date_password_modified
//                 .unwrap_or_default()
//         );
//         buf_writer
//             .write_all(pass_str.as_bytes())
//             .await
//             .into_diagnostic()?;
//     }
//     buf_writer
//         .flush()
//         .await
//         .into_diagnostic()?;
//     buf_writer
//         .get_ref()
//         .sync_all()
//         .await
//         .into_diagnostic()?;
//     println!("{} password done", getter.browser());
//
//     Ok(())
// }

// async fn write_chromium_cookies<T: ChromiumInfo + Sync + Send>(
//     getter: &ChromiumGetter<T>,
// ) -> Result<()> {
//     println!("{} cookies", getter.browser());
//     let getter = ChromiumBuilder::<Chrome>::new()
//         .build()
//         .await?;
//     let cks = getter.get_cookies_all().await?;
//     let mut buf_writer = open_file(getter.browser(), "cookies").await?;
//     let head  = b"Url,Name,Path,Value,DecryptedValue,IsSecure,IsHttponly,SourcePort,CreationUtc,ExpiresUtc,LastAccessUtc,LastUpdateUtc,HasExpires,IsPersistent\n";
//     buf_writer
//         .write_all(head)
//         .await
//         .into_diagnostic()?;
//     for ck in cks {
//         let ck_str = format!(
//             "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
//             ck.host_key,
//             ck.name,
//             ck.path,
//             ck.value,
//             ck.decrypted_value
//                 .unwrap_or_default(),
//             ck.is_secure,
//             ck.is_httponly,
//             ck.source_port,
//             ck.creation_utc.unwrap_or_default(),
//             ck.expires_utc.unwrap_or_default(),
//             ck.last_access_utc
//                 .unwrap_or_default(),
//             ck.last_update_utc
//                 .unwrap_or_default(),
//             ck.has_expires,
//             ck.is_persistent,
//         );
//         buf_writer
//             .write_all(ck_str.as_bytes())
//             .await
//             .into_diagnostic()?;
//     }
//
//     buf_writer
//         .flush()
//         .await
//         .into_diagnostic()?;
//     buf_writer
//         .get_ref()
//         .sync_all()
//         .await
//         .into_diagnostic()?;
//     println!("{} cookies done", getter.browser());
//
//     Ok(())
// }

// async fn write_firefox_cookies<T>(getter: &FirefoxGetter<T>) -> Result<()>
// where
//     T: FirefoxInfo + Send + Sync,
// {
//     println!("{} cookies", getter.browser());
//     let head  = b"Host,Name,Path,Value,CreationTime,LastAccessed,Expiry,IsSecure,IsHttpOnly,OriginAttributes";
//     let mut buf_writer = open_file(getter.browser(), "cookies").await?;
//     buf_writer
//         .write_all(head)
//         .await
//         .into_diagnostic()?;
//     let cks = getter.get_cookies_all().await?;
//     for ck in cks {
//         let ck_str = format!(
//             "{},{},{},{},{},{},{},{},{},{}\n",
//             ck.host,
//             ck.name,
//             ck.path,
//             ck.value,
//             ck.creation_time
//                 .unwrap_or_default(),
//             ck.last_accessed
//                 .unwrap_or_default(),
//             ck.expiry.unwrap_or_default(),
//             ck.is_secure,
//             ck.is_http_only,
//             ck.origin_attributes
//         );
//         buf_writer
//             .write_all(ck_str.as_bytes())
//             .await
//             .into_diagnostic()?;
//     }
//
//     buf_writer
//         .flush()
//         .await
//         .into_diagnostic()?;
//     buf_writer
//         .get_ref()
//         .sync_all()
//         .await
//         .into_diagnostic()?;
//     println!("{} cookies done", getter.browser());
//     Ok(())
// }

// #[cfg(target_os = "macos")]
// async fn write_safari_cookies(getter: &SafariGetter) -> Result<()> {
//     println!("{} cookies", getter.browser());
//     let head = b"Domain,Name,Path,Value,Creation,Expires,IsSecure,IsHttpOnly,Comment\n";
//     let mut buf_writer = open_file(getter.browser(), "cookies").await?;
//     buf_writer
//         .write_all(head)
//         .await
//         .into_diagnostic()?;
//     let cks = getter.all_cookies();
//     for ck in cks {
//         let ck_str = format!(
//             "{},{},{},{},{},{},{},{},{}\n",
//             ck.domain,
//             ck.name,
//             ck.path,
//             ck.value,
//             ck.creation.unwrap_or_default(),
//             ck.expires.unwrap_or_default(),
//             ck.is_secure,
//             ck.is_httponly,
//             ck.comment,
//         );
//         buf_writer
//             .write_all(ck_str.as_bytes())
//             .await
//             .into_diagnostic()?;
//     }
//
//     buf_writer
//         .flush()
//         .await
//         .into_diagnostic()?;
//     buf_writer
//         .get_ref()
//         .sync_all()
//         .await
//         .into_diagnostic()?;
//     println!("{} cookies done", getter.browser());
//     Ok(())
// }
//
// pub async fn run() -> Result<()> {
//     create_dir_all(BASE_DIR)
//         .await
//         .into_diagnostic()?;
//     let mut jds: Vec<tokio::task::JoinHandle<()>> = vec![];
//
//     // for browser in Browser::iter() {
//     //     match browser {
//     //         Browser::Firefox | Browser::Librewolf => {
//     //             let hd = tokio::spawn(async move {
//     //                 let getter = match FirefoxBuilder::new(Firefox::new().unwrap())
//     //                     .build()
//     //                     .await
//     //                 {
//     //                     Ok(it) => it,
//     //                     Err(err) => {
//     //                         tracing::warn!("Firefox Getter wrong: {err}");
//     //                         return;
//     //                     },
//     //                 };
//     //                 match write_firefox_cookies(&getter).await {
//     //                     Ok(()) => {},
//     //                     Err(err) => tracing::warn!("{err}"),
//     //                 }
//     //             });
//     //             jds.push(hd);
//     //         },
//     //         #[cfg(target_os = "macos")]
//     //         Browser::Safari => {
//     //             use decrypt_cookies::SafariBuilder;
//     //             let hd = tokio::spawn(async move {
//     //                 let getter = match SafariBuilder::new().build().await {
//     //                     Ok(it) => it,
//     //                     Err(err) => {
//     //                         tracing::warn!("{browser} wrong: {err}");
//     //                         return;
//     //                     },
//     //                 };
//     //                 match write_safari_cookies(&getter).await {
//     //                     Ok(()) => {},
//     //                     Err(err) => tracing::warn!("{browser} wrong: {err}"),
//     //                 };
//     //             });
//     //             jds.push(hd);
//     //         },
//     //
//     //         browser => {
//     //             let hd = tokio::spawn(async move {
//     //                 let getter = match ChromiumBuilder::new(Chrome::new())
//     //                     .build()
//     //                     .await
//     //                 {
//     //                     Ok(it) => it,
//     //                     Err(err) => {
//     //                         tracing::warn!("{browser} wrong: {err}");
//     //                         return;
//     //                     },
//     //                 };
//     //                 match write_chromium_cookies(&getter).await {
//     //                     Ok(()) => {},
//     //                     Err(err) => tracing::warn!("{browser} Cookies wrong: {err}"),
//     //                 };
//     //                 match write_chromium_password(&getter).await {
//     //                     Ok(()) => {},
//     //                     Err(err) => tracing::warn!("{browser} Cookies wrong: {err}"),
//     //                 };
//     //             });
//     //             jds.push(hd);
//     //         },
//     //     }
//     // }
//     for ele in jds {
//         match ele.await {
//             Ok(()) => {},
//             Err(err) => tracing::warn!("{err}"),
//         }
//     }
//
//     Ok(())
// }
