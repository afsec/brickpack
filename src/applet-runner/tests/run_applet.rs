// use applet_runner::{AppletRunner, AppletRunnerArtifact};
// use application_models::applets::{
//     model::{AppletCode, AppletFilename, AppletOid},
//     Applet,
// };
// use design_scaffold::oid::ObjectIdReactor;
// use http::Method;

// #[tokio::test]
// async fn ok_on_try_run_hello() {
//     let code_in_base64 = "ZWNobygiSGVsbG8gd29ybGQhIik=".to_string();
//     //                            echo("Hello world!")

//     let oid_generator = ObjectIdReactor::new().await.unwrap().take();
//     let new_applet_oid = AppletOid::new(Some(&oid_generator)).await.unwrap();
//     let filename = AppletFilename::from("hello.lua".to_string());
//     let code = AppletCode::from(code_in_base64);
//     let applet = Applet::new(new_applet_oid, filename, code);
//     let artifact = AppletRunnerArtifact::from(applet);
//     let applet = AppletRunner::new(Method::GET, None, None, None, artifact);
//     let result = applet.run();

//     match &result {
//         Ok(outcome) => {
//             println!("{outcome}");
//         }
//         Err(error) => {
//             dbg!(error);
//         }
//     }
//     assert!(result.is_ok());
// }

// #[tokio::test]
// async fn ok_on_try_run_render_html() {
//     use applet_runner::{AppletRunner, AppletRunnerArtifact};
//     use application_models::applets::{
//         model::{AppletCode, AppletFilename},
//         Applet,
//     };
//     let code_in_base64 = "bG9jYWwgeCA9IDQyCmVjaG8oIjxoMT5JdCB3b3JrcyE8L2gxPiIpCmVjaG8oIjx1bD4iKQplY2hvKCI8bGk+SGVsbG8gIikKbG9jYWwgb3V0cHV0ID0gdG9zdHJpbmcoeCkgCmVjaG8ob3V0cHV0KQplY2hvKCI8L2xpPiIpCnggPSB4ICsgMQplY2hvKCI8bGk+SGVsbG8gIikKbG9jYWwgb3V0cHV0ID0gdG9zdHJpbmcoeCkgCmVjaG8ob3V0cHV0KQplY2hvKCI8L2xpPiIpCmVjaG8oIjwvdWw+IikK".to_string();
//     /*
//         local x = 42
//         echo("<h1>It works!</h1>")
//         echo("<ul>")
//         echo("<li>Hello ")
//         local output = tostring(x)
//         echo(output)
//         echo("</li>")
//         x = x + 1
//         echo("<li>Hello ")
//         local output = tostring(x)
//         echo(output)
//         echo("</li>")
//         echo("</ul>")
//     */
//     let oid_generator = ObjectIdReactor::new().await.unwrap().take();
//     let new_applet_oid = AppletOid::new(Some(&oid_generator)).await.unwrap();
//     let filename = AppletFilename::from("hello.lua".to_string());
//     let code = AppletCode::from(code_in_base64);
//     let applet = Applet::new(new_applet_oid, filename, code);
//     let artifact = AppletRunnerArtifact::from(applet);
//     let applet = AppletRunner::new(Method::GET, None, None, None, artifact);
//     let result = applet.run();

//     match &result {
//         Ok(outcome) => {
//             println!("{outcome}");
//         }
//         Err(error) => {
//             dbg!(error);
//         }
//     }
//     assert!(result.is_ok());
// }

// #[tokio::test]
// async fn err_on_try_run_code_with_invalid_function() {
//     use applet_runner::{AppletRunner, AppletRunnerArtifact};
//     use application_models::applets::{
//         model::{AppletCode, AppletFilename},
//         Applet,
//     };
//     let code_in_base64 = "bG9jYWwgeCA9IDQyCmVjaG8oIjxoMT5JdCB3b3JrcyE8L2gxPiIpCmVjaG8oIjx1bD4iKQplY2hvKCI8bGk+SGVsbG8gIikKbG9jYWwgb3V0cHV0ID0gdG9fc3RyaW5nKHgpIAplY2hvKG91dHB1dCkKZWNobygiPC9saT4iKQp4ID0geCArIDEKZWNobygiPGxpPkhlbGxvICIpCmxvY2FsIG91dHB1dCA9IHRvc3RyaW5nKHgpIAplY2hvKG91dHB1dCkKZWNobygiPC9saT4iKQplY2hvKCI8L3VsPiIpCg==".to_string();
//     /*
//         local x = 42
//         echo("<h1>It works!</h1>")
//         echo("<ul>")
//         echo("<li>Hello ")
//         local output = to_string(x) <------------ ERROR
//         echo(output)
//         echo("</li>")
//         x = x + 1
//         echo("<li>Hello ")
//         local output = tostring(x)
//         echo(output)
//         echo("</li>")
//         echo("</ul>")
//     */
//     let oid_generator = ObjectIdReactor::new().await.unwrap().take();
//     let new_applet_oid = AppletOid::new(Some(&oid_generator)).await.unwrap();
//     let filename = AppletFilename::from("hello.lua".to_string());
//     let code = AppletCode::from(code_in_base64);
//     let applet = Applet::new(new_applet_oid, filename, code);
//     let artifact = AppletRunnerArtifact::from(applet);
//     let applet = AppletRunner::new(Method::GET, None, None, None, artifact);
//     let result = applet.run();
//     match &result {
//         Ok(outcome) => {
//             println!("{outcome}");
//         }
//         Err(error) => {
//             dbg!(error);
//         }
//     }
//     assert!(result.is_err());
// }
