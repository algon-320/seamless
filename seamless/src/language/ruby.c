#include "ruby.h"
#include "ruby/encoding.h"

void seamless_ruby_init(void)
{
    RUBY_INIT_STACK;
    ruby_init();
    ruby_init_loadpath();
    rb_enc_find_index("encdb");
    rb_require("rubygems");
}
void semaless_ruby_require(const char *file)
{
    rb_require(file);
}
void seamless_ruby_finalize(void)
{
    ruby_finalize();
}

VALUE seamless_ruby_int2fix(int x)
{
    return INT2FIX(x);
}
VALUE seamless_ruby_long2fix(long x)
{
    return LONG2FIX(x);
}
int seamless_ruby_num2int(VALUE num)
{
    return NUM2INT(num);
}
long seamless_ruby_num2long(VALUE num)
{
    return NUM2LONG(num);
}

#include <stdio.h>
VALUE seamless_ruby_call(const char *func_name, int argc, const VALUE *argv)
{
    return rb_funcall2(rb_cObject, rb_intern(func_name), argc, argv);
}
