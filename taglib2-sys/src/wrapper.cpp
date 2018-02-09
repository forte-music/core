#include <string>
#include "../taglib/taglib/tag.h"
#include "../taglib/taglib/fileref.h"
#include "wrapper.hpp"

extern "C" {
    const char *hello(const char *fileName) {
        std::string helloStr("hello world, ");
        std::string fileNameStr(fileName);
        std::string *out = new std::string(helloStr + fileNameStr);
        return out->c_str();
    }
}
