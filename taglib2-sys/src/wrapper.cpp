#include <string>
#include "../taglib/taglib/tag.h"
#include "../taglib/taglib/fileref.h"
#include "wrapper.hpp"

extern "C" {
    const char *hello(const char *fileName) {
        TagLib::FileRef file((TagLib::FileName) fileName);
        if(file.isNull() || !file.tag()) {
            return "failed";
        }

        TagLib::Tag *tag = file.tag();
        TagLib::PropertyMap properties = tag->properties();

        return properties.toString().toCString();
    }
}
