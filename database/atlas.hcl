env "dev" {
  src = "file://schema"
  dev = "docker://postgres/15/dev?search_path=public"
  
  migration {
    dir    = "file://migrations"
    format = golang-migrate
  }
  format {
    migrate {
      diff = "{{ sql . \"  \" }}"
    } 
  }
}

diff {
  skip {
    // By default, none of the changes are skipped.
    drop_schema = true
    drop_table  = true
  }
}