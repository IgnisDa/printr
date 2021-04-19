<a name=""></a>
##  (2021-04-19)


#### Bug Fixes

* **cli.yaml:**  add description, remove option from format ([92ea1c31](92ea1c31))
* **main.rs:**  reorder passing arguments ([d18adf4b](d18adf4b))
* **printr.1.txt.tpl, printr.txt.tpl:**  rename file ([0ef554c0](0ef554c0))
* **standard-jobs.yml, ubuntu-install-packages:**  rename file, change references ([487d8286](487d8286))

#### Features

*   initial commit ([fa32ce55](fa32ce55))
* **.gitignore:**  rewrite to correct files ([b58e3523](b58e3523))
* **app.rs, cli.rs, cli.yaml:**  rename file ([7c8942a1](7c8942a1))
* **build.rs, generate-manpage, string-substitution.py:**  delete files, replace with build.rs ([22bae3f8](22bae3f8))
* **cli.rs:**
  *  return the args ([470a2863](470a2863))
  *  module to parse from command line ([441ab2a4](441ab2a4))
* **cli.yaml:**
  *  add formatting option ([24e2a50d](24e2a50d))
  *  add `error` flag ([9a51d573](9a51d573))
  *  add plain, color option ([99116540](99116540))
  *  file to define command line structure ([aca186fa](aca186fa))
* **lib.rs:**
  *  rename file, add functionality ([ac1ae196](ac1ae196))
  *  module to contain essential functions ([3832dbe4](3832dbe4))
* **main.rs:**
  *  adapt to new interface ([9c5d7b48](9c5d7b48))
  *  complete parsing args ([d6d3f1bc](d6d3f1bc))
  *  create printr obj from command line ([6d14f0be](6d14f0be))
  *  add call to command line ([b466f9e3](b466f9e3))
* **mod.rs:**
  *  complete almost all implementations ([57641e04](57641e04))
  *  add sentiment analysis methods ([e5d0a93e](e5d0a93e))
* **tests.rs:**  delete useless file ([f19b2aad](f19b2aad))
