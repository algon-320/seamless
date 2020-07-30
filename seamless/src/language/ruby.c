#include "ruby.h"
#include "ruby/encoding.h"

void seamless_language_ruby_init()
{
    ruby_init();
    ruby_init_loadpath();
    rb_enc_find_index("encdb"); // encodingライブラリの初期化
    rb_require("rubygems");
    rb_require("./ruby_test");
}

void foo(VALUE x)
{
    VALUE module = rb_const_get(rb_cObject, rb_intern("Test"));
    VALUE klass = rb_const_get(module, rb_intern("Foo"));
    VALUE obj = rb_class_new_instance(0, NULL, klass);
    rb_funcall(obj, rb_intern("foo"), 1, x);
}

// int main()
// {
//     init();
//     // foo(INT2FIX(12345));
//     foo(rb_str_new2("hello"));
// }