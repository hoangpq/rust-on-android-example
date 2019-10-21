#ifndef JNI_WRAPPER_H_
#define JNI_WRAPPER_H_

#include <iostream>
#include <jni.h>

#include "../util/util.h"
#include "object_wrap.h"
#include "v8.h"
#include <android/looper.h>

using namespace v8;

extern "C" {
jlong get_current_activity();
bool is_method(jlong, string_t);
bool is_field(jlong, string_t);
}

extern "C" int looperCallback(int fd, int events, void *data);

class JavaWrapper : public rust::ObjectWrap {

public:
  static void Init(Isolate *isolate_, Local<ObjectTemplate> exports);

    static void SetContext(Local<Context> context_);

private:
    explicit JavaWrapper(std::string package) : package_(package) {};

    ~JavaWrapper();

  static void New(const FunctionCallbackInfo<Value> &args);

    static void IsField(const FunctionCallbackInfo<Value> &args);

    static void IsMethod(const FunctionCallbackInfo<Value> &args);

  static void Call(const FunctionCallbackInfo<Value> &args);

    static void InvokeJavaFunction(const FunctionCallbackInfo<Value> &args);

  std::string package_;
    bool context_ = false;
    jlong ptr_;

  static Persistent<FunctionTemplate> constructor_;
    static Persistent<Function> registerUITask_;
    static Persistent<Function> resolverUITask;
    static Persistent<Context> resolverContext_;
};

#endif // JNI_WRAPPER_H_