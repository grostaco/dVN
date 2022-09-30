use std::{path::Path, rc::Rc};

use reqwest::Client;
use web_sys::MouseEvent;
use yew::{html, Callback, Html};

pub async fn get_files(client: Rc<Client>) -> Vec<String> {
    client
        .get("http://127.0.0.1:8000/api/files")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .split(',')
        .map(ToOwned::to_owned)
        .collect()
}

pub async fn get_file(client: Rc<Client>, file: &str) -> String {
    client
        .get(format!("http://127.0.0.1:8000/{file}"))
        .header("Content-Length", 4096)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

pub async fn post_file(client: Rc<Client>, file: &str, content: String) -> String {
    client
        .post(format!("http://127.0.0.1:8000/api/file/{file}"))
        .body(content)
        .send()
        .await
        .unwrap();

    String::new()
}

pub fn file_tree(
    files: Vec<&'_ Path>,
    expand_callback: &Callback<MouseEvent>,
    file_callback: &Callback<MouseEvent>,
) -> Html {
    let mut folders = Vec::new();
    let mut folder_files = Vec::new();

    let mut i = 1;
    while i < files.len() {
        let file = files.get(i).unwrap();
        if file.extension().is_none() {
            let sub_files = files
                .iter()
                .skip(i)
                .take_while(|p| p.starts_with(file))
                .copied()
                .collect::<Vec<_>>();
            i += sub_files.len() - 1;
            folders.push(file_tree(sub_files, expand_callback, file_callback));
        } else {
            folder_files.push(
                html! { <div path={file.to_str().unwrap().to_string()} onclick={file_callback}>{file.file_name().unwrap().to_str().unwrap()}</div> },
            );
        }
        i += 1;
    }

    html! {
        <>
        if let Some(file) = files.first() {
            <div class="name" onclick={expand_callback}>{ file.file_name().unwrap().to_str().unwrap() }</div>
            <div class="children">
                if !folders.is_empty() {
                    <div class="folder">
                        {for folders}
                    </div>
                }

                if !folder_files.is_empty() {
                    <div class="files">
                        {for folder_files}
                    </div>
                }
            </div>
        }
        </>
    }
}

// #[cfg(test)]
// mod test {
//     use std::path::Path;

//     use yew::{html, Html};

//     fn file_tree(files: Vec<&'_ Path>) -> Html {
//         println!("called with: {:?}", files);
//         let mut folders = Vec::new();
//         let mut folder_files = Vec::new();

//         let mut i = 1;
//         while i < files.len() {
//             let file = files.get(i).unwrap();
//             if file.extension().is_none() {
//                 // let mut sub_files = Vec::new();
//                 // sub_files.push()
//                 let sub_files = files
//                     .iter()
//                     .skip(i)
//                     .take_while(|p| {
//                         //println!("{:?} {:?} {}", p, file, p.starts_with(file));
//                         p.starts_with(file)
//                     })
//                     .copied()
//                     .collect::<Vec<_>>();

//                 //println!("sub_files: {:?}", sub_files);
//                 i += sub_files.len() - 1;
//                 println!("skipped to: {}", i);
//                 folders.push(file_tree(sub_files));
//             } else {
//                 //println!("{:?} {:?}", file, files);
//                 folder_files
//                     .push(html! { <div>{file.file_name().unwrap().to_str().unwrap()}</div> });
//             }

//             i += 1;
//         }
//         //println!("{:?}", files);
//         // for (i, file) in files.iter().enumerate() {

//         // }

//         html! {
//             <>
//             if let Some(file) = files.first() {
//                 <div class="name">{ file.to_str().unwrap() }</div>
//                 <div class="children">
//                     if !folders.is_empty() {
//                         <div class="folder">
//                             {for folders}
//                         </div>
//                     }

//                     if !folder_files.is_empty() {
//                         <div class="files">
//                             {for folder_files}
//                         </div>
//                     }
//                 </div>
//             }
//             </>
//         }
//     }

//     #[test]
//     fn file_tree_test() {
//         let files = r"foo,foo/a,foo/a/a.txt,foo/a/b.txt,foo/c,foo/c/c.txt,foo/b.txt"
//             .split(',')
//             .map(Path::new)
//             .collect::<Vec<_>>();
//         file_tree(files);
//         //println!("{:#?}", file_tree(files, &mut s));
//     }
// }
