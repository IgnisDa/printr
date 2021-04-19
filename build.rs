// Adapted from https://github.com/BurntSushi/ripgrep/blob/master/build.rs

use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::process;

use clap::Shell;

#[path = "src/app.rs"]
mod app;
use app::{CustomArg, CustomArgKind};

fn main() {
    // OUT_DIR is set by Cargo and it's where any additional build artifacts
    // are written.
    let outdir = if let Some(outdir) = env::var_os("OUT_DIR") {
        outdir
    } else {
        eprintln!(
            "OUT_DIR environment variable not defined. \
                 Please file a bug: \
                 https://github.com/IgnisDa/printr/issues/new"
        );
        process::exit(1);
    };
    if fs::remove_dir_all(&outdir).is_ok() {};
    fs::create_dir_all(&outdir).unwrap();
    let stamp_path = Path::new(&outdir).join("printr-stamp");
    if let Err(err) = File::create(&stamp_path) {
        panic!("failed to write {}: {}", stamp_path.display(), err);
    }
    if let Err(err) = generate_man_page(&outdir) {
        eprintln!("failed to generate man page: {}", err);
    }
    // generate completions
    let mut app = app::app();
    app.gen_completions("printr", Shell::Bash, &outdir);
    app.gen_completions("printr", Shell::Fish, &outdir);
    app.gen_completions("printr", Shell::Zsh, &outdir);
    app.gen_completions("printr", Shell::PowerShell, &outdir);
}

fn generate_man_page<P: AsRef<Path>>(outdir: P) -> io::Result<()> {
    // 1. Read asciidoctor template.
    // 2. Interpolate template with auto-generated docs.
    // 3. Save interpolation to disk.
    // 4. Use asciidoctor to convert to man page.
    let outdir = outdir.as_ref();
    let cwd = env::current_dir()?;
    let tpl_path = cwd.join("docs").join("printr.1.txt.tpl");
    let txt_path = outdir.join("printr.1.txt");
    let mut tpl = String::new();
    File::open(&tpl_path)?.read_to_string(&mut tpl)?;
    let options = formatted_options()?
        .replace("&#123;", "{")
        .replace("&#125;", "}");
    tpl = tpl.replace("{{OPTIONS}}", &options);
    tpl = tpl.replace("{{VERSION}}", clap::crate_version!());
    tpl = tpl.replace("{{AUTHORS}}", clap::crate_authors!(",\n"));
    File::create(&txt_path)?.write_all(tpl.as_bytes())?;

    let result = process::Command::new("asciidoctor")
        .arg("--doctype")
        .arg("manpage")
        .arg("--backend")
        .arg("manpage")
        .arg(&txt_path)
        .spawn()?
        .wait()?;
    if !result.success() {
        let msg = format!("'asciidoctor' failed with exit code {:?}", result.code());
        return Err(ioerr(msg));
    }
    Ok(())
}

fn formatted_options() -> io::Result<String> {
    let mut args = app::all_args_and_flags();
    args.sort_by(|x1, x2| x1.name.cmp(&x2.name));

    let mut formatted = vec![];
    for arg in args {
        if arg.hidden {
            continue;
        }
        if let app::CustomArgKind::Positional { .. } = arg.kind {
            continue;
        }
        formatted.push(formatted_arg(&arg)?);
    }
    Ok(formatted.join("\n\n"))
}

fn formatted_arg(arg: &CustomArg) -> io::Result<String> {
    match arg.kind {
        CustomArgKind::Positional { .. } => {
            panic!("unexpected positional argument")
        }
        CustomArgKind::Switch {
            long,
            short,
            multiple,
        } => {
            let mut out = vec![];

            let mut header = format!("--{}", long);
            if let Some(short) = short {
                header = format!("-{}, {}", short, header);
            }
            if multiple {
                header = format!("*{}* ...::", header);
            } else {
                header = format!("*{}*::", header);
            }
            writeln!(out, "{}", header)?;
            writeln!(out, "{}", formatted_doc_txt(arg)?)?;

            Ok(String::from_utf8(out).unwrap())
        }
        CustomArgKind::Flag {
            long,
            short,
            value_name,
            multiple,
            ..
        } => {
            let mut out = vec![];

            let mut header = format!("--{}", long);
            if let Some(short) = short {
                header = format!("-{}, {}", short, header);
            }
            if multiple {
                header = format!("*{}* _{}_ ...::", header, value_name);
            } else {
                header = format!("*{}* _{}_::", header, value_name);
            }
            writeln!(out, "{}", header)?;
            writeln!(out, "{}", formatted_doc_txt(arg)?)?;

            Ok(String::from_utf8(out).unwrap())
        }
    }
}

fn formatted_doc_txt(arg: &CustomArg) -> io::Result<String> {
    let paragraphs: Vec<String> = arg
        .doc_long
        .replace("{", "&#123;")
        .replace("}", r"&#125;")
        // Hack to render ** literally in man page correctly. We can't put
        // these crazy +++ in the help text directly, since that shows
        // literally in --help output.
        .replace("*-g 'foo/**'*", "*-g +++'foo/**'+++*")
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();
    if paragraphs.is_empty() {
        return Err(ioerr(format!("missing docs for --{}", arg.name)));
    }
    let first = format!("  {}", paragraphs[0].replace("\n", "\n  "));
    if paragraphs.len() == 1 {
        return Ok(first);
    }
    Ok(format!("{}\n+\n{}", first, paragraphs[1..].join("\n+\n")))
}

fn ioerr(msg: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, msg)
}
