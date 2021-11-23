# Introduction of dessert2
`dessert2` is a tool that extracts text from HTML document. A list of values could be fetched in one round. [css selectors](https://www.w3schools.com/cssref/css_selectors.asp) are used for identify the html dom nodes that you want to fetch. The program read your request from the parameter `--template` or `--template-file`, read the html text from either `--url` or the stdin or the pipe, out the result in either `json`, `yaml`, or text.

# Use of dessert2
Either `--template` or `--template-file` must appear in the parameter list, so that the program knows your demand. It could be in either `json` or `yaml`. 
If the parameter `--url` is absent, it will be read from stdin or through the pipe.
The `--output-format` could be `yaml`, `json` or `text`. The default value is `yaml`.

#The simplest use case
```
dessert2 --url https://en.wikipedia.org/wiki/List_of_cities_in_Canada --template '[{"object_id":"cities","css_selector":"div#toc a","properties":[{"id": "number","css_selector":"span.tocnumber","value_type":"Str","value_from":"InnerText"},{"id": "province","css_selector":"span.toctext","value_type":"Str","value_from":"InnerText"}]}]' --ouput-format yaml
```
# License
This program is under MIT license.
