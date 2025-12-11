[‼️]: ✏️README.mdt

# @w5/camel

[test/main.coffee](./test/main.coffee) :

```coffee
#!/usr/bin/env coffee

import camel from '@w5/camel'

for i in 'index.htm str_li str2li i18n iI18n test-db User LIB_INDEX'.split(' ')
  console.log "#{i} → #{camel i}"
```

output :

```
index.htm → index.htm
str_li → strLi
str2li → str2li
i18n → i18n
iI18n → iI18n
test-db → testDb
User → User
LIB_INDEX → LIB_INDEX
```
