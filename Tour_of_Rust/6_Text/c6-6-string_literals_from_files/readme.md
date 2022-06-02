# String Literals From Files

If you have some very large text, consider using the macro **include_str!** to include
text from local files in your program:

let hello_html = include_str!("hello.html")
