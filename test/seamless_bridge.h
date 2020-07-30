#ifndef SEAMLESS_BRIDGE_H
#define SEAMLESS_BRIDGE_H

#define def_bridge_0(func_name, ret_type)        \
    void func_name##_bridge(void *return_buf)    \
    {                                            \
        *(ret_type *)(return_buf) = func_name(); \
    }
#define def_bridge_1(func_name, ret_type, a1)             \
    void func_name##_bridge(void *return_buf, void *x1)   \
    {                                                     \
        *(ret_type *)(return_buf) = func_name(*(a1 *)x1); \
    }
#define def_bridge_2(func_name, ret_type, a1, a2)                    \
    void func_name##_bridge(void *return_buf, void *x1, void *x2)    \
    {                                                                \
        *(ret_type *)(return_buf) = func_name(*(a1 *)x1, *(a2 *)x2); \
    }

#define def_bridge_void_0(func_name)                \
    void func_name##_bridge(void *dummy_return_buf) \
    {                                               \
        *(ret_type *)(return_buf) = func_name();    \
    }
#define def_bridge_void_1(func_name, a1)                      \
    void func_name##_bridge(void *dummy_return_buf, void *x1) \
    {                                                         \
        func_name(*(a1 *)x1);                                 \
    }
#define def_bridge_void_2(func_name, a1, a2)                            \
    void func_name##_bridge(void *dummy_return_buf, void *x1, void *x2) \
    {                                                                   \
        func_name(*(a1 *)x1, *(a2 *)x2);                                \
    }

#endif