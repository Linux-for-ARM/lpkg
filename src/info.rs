//! Print information about software

use std::process::exit;

use colored::Colorize;

use crate::db::Db;
use crate::error::*;
use crate::{err_msg, msg};

pub fn list_all(ver: bool, arch: bool) -> Result<()> {
    let pkgs_all = Db::open()?.entries;

    for pkg in &pkgs_all {
        print!("{}", pkg.0);
        if ver {
            print!(" {}", &pkg.1.package.version);
        }
        if arch {
            print!(" {}", &pkg.1.package.arch);
        }
        println!();
    }

    Ok(())
}

pub fn info(pkgname: &str) -> Result<()> {
    let pkgs_db = Db::open()?.entries;
    let pkgs_all = pkgs_db.get(pkgname);

    match pkgs_all {
        Some(pkg) => {
            msg!("Package «{}» metadata:", pkgname.bold());

            print_info("name", &pkg.package.name);
            print_info("version", &pkg.package.version);
            print_info("summary", &pkg.package.summary);
            print_info("maintainer", &pkg.package.maintainer);
            print_info("architecture", &pkg.package.arch);
            print_info("install prefix", &pkg.install_prefix);

            if let Some(deps) = &pkg.deps {
                println!();
                msg!("Package «{}» dependencies:", pkgname.bold());

                if let Some(required) = &deps.required {
                    /* Формируем строку со списком зависимостей. Имя пакета
                     * подсвечивается красным, если этого пакета нет в базе
                     * данных. И не подсвечивается, если пакет в БД присутствует
                     */
                    let mut req_str = String::new();
                    for dep in required {
                        if pkgs_db.get(dep).is_none() {
                            req_str = format!("{req_str}{} ", dep.red());
                        } else {
                            req_str = format!("{req_str}{} ", dep);
                        }
                    }
                    print_info("required", &req_str);
                }
                if let Some(optional) = &deps.optional {
                    // Формирование этой строки аналогично необходимым зависимостям
                    let mut opt_str = String::new();
                    for dep in optional {
                        if pkgs_db.get(dep).is_none() {
                            opt_str = format!("{opt_str}{} ", dep.red());
                        } else {
                            opt_str = format!("{opt_str}{} ", dep);
                        }
                    }
                    print_info("optional", &opt_str);
                }
            }
        }
        None => {
            err_msg!("Failed to get information about «{pkgname}»!");
            exit(1);
        }
    }

    Ok(())
}

fn print_info(hdr: &str, data: &str) {
    println!("{:>width$} {}", format!("{hdr}:").bold(), data, width = 15);
}
