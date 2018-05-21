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
}

void read_artwork(SongProperties *song, TagLib::PictureMap &map) {
    TagLib::Picture::Type options[] = {TagLib::Picture::Type::FrontCover, TagLib::Picture::Type::Other};

    for (TagLib::Picture::Type option : options) {
        if (map.contains(option)) {
            TagLib::Picture picture = map[option].front();
            song->picture_mime = to_cstr(picture.mime());

            // We need to copy the picture data manually because it's not a string
            TagLib::ByteVector pictureData = picture.data();
            size_t pictureSize = pictureData.size();
            song->picture_data = (char*) malloc(pictureSize);
            memcpy(song->picture_data, pictureData.data(), pictureSize);
            song->picture_data_len = pictureSize;

            return;
        }
    }
}

extern "C" {
    SongProperties *song_properties(const char *fileName) {
        TagLib::FileRef file((TagLib::FileName) fileName);

        // Check if the file was opened
        if(file.isNull() || !file.tag()) {
            return NULL;
        }

        // Read off song properties
        SongProperties *song_properties = new SongProperties();

        TagLib::Tag *tag = file.tag();
        TagLib::AudioProperties *audioProperties = file.audioProperties();
        TagLib::PropertyMap properties = tag->properties();

        song_properties->title = to_cstr(tag->title());
        song_properties->album = to_cstr(tag->album());
        song_properties->artist = to_cstr(tag->artist());

        // There are multiple ways the album artist can be stored.
        if(properties.contains("ALBUMARTIST")) {
            song_properties->album_artist = to_cstr(properties["ALBUMARTIST"].toString());
        } else if(properties.contains("ALBUM_ARTIST")) {
            song_properties->album_artist = to_cstr(properties["ALBUM_ARTIST"].toString());
        } else if(properties.contains("ALBUM ARTIST")) {
            song_properties->album_artist = to_cstr(properties["ALBUM ARTIST"].toString());
        }

        song_properties->year = tag->year();
        song_properties->track_number = tag->track();
        song_properties->duration = audioProperties->length();

        TagLib::PictureMap map = tag->pictures();
        read_artwork(song_properties, map);

        return song_properties;
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
