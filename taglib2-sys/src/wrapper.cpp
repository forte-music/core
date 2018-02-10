#include <string.h>
#include "../taglib/taglib/tag.h"
#include "../taglib/taglib/fileref.h"
#include "../taglib/taglib/toolkit/tpicturemap.h"

char *to_cstr(TagLib::String str) {
    return strdup(str.toCString(true));
}

extern "C" {
    typedef struct {
        char *title;
        char *album;
        char *artist;
        char *album_artist;
        unsigned int year;
        unsigned int track_number;
        int duration;
        char *picture_data;
        unsigned int picture_data_len;
        char *picture_mime;
    } SongProperties;

    SongProperties *song_properties(const char *fileName) {
        TagLib::FileRef file((TagLib::FileName) fileName);

        // Check if the file was opened
        if(file.isNull() || !file.tag()) {
            return NULL;
        }

        // Read off song properties
        SongProperties *songProperties = new SongProperties();

        TagLib::Tag *tag = file.tag();
        TagLib::AudioProperties *audioProperties = file.audioProperties();
        TagLib::PropertyMap properties = tag->properties();

        songProperties->title = to_cstr(tag->title());
        songProperties->album = to_cstr(tag->album());
        songProperties->artist = to_cstr(tag->artist());

        // There are multiple ways the album artist can be stored.
        if(properties.contains("ALBUMARTIST")) {
            songProperties->album_artist = to_cstr(properties["ALBUMARTIST"].toString());
        } else if(properties.contains("ALBUM_ARTIST")) {
            songProperties->album_artist = to_cstr(properties["ALBUM_ARTIST"].toString());
        } else if(properties.contains("ALBUM ARTIST")) {
            songProperties->album_artist = to_cstr(properties["ALBUM ARTIST"].toString());
        }

        songProperties->year = tag->year();
        songProperties->track_number = tag->track();
        songProperties->duration = audioProperties->length();

        TagLib::Picture picture = tag->pictures()[
                TagLib::Picture::Type::FrontCover
        ].front();
        songProperties->picture_mime = to_cstr(picture.mime());

        // We need to copy the picture data manually because it's not a string
        TagLib::ByteVector pictureData = picture.data();
        size_t pictureSize = pictureData.size();
        songProperties->picture_data = (char*) malloc(pictureSize);
        memcpy(songProperties->picture_data, pictureData.data(), pictureSize);
        songProperties->picture_data_len = pictureSize;

        return songProperties;
    }

    void destroy_properties(SongProperties *songProperties) {
        free(songProperties->title);
        free(songProperties->album);
        free(songProperties->artist);
        free(songProperties->album_artist);
        free(songProperties->picture_data);
        free(songProperties->picture_mime);
        delete songProperties;
    }
}
