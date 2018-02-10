#include <string.h>
#include "../taglib/taglib/tag.h"
#include "../taglib/taglib/fileref.h"
#include "../taglib/taglib/toolkit/tpicturemap.h"

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
        SongProperties *songProperties = new SongProperties();

        if(file.isNull() || !file.tag()) {
            return songProperties;
        }

        TagLib::Tag *tag = file.tag();
        TagLib::AudioProperties *audioProperties = file.audioProperties();
        TagLib::PropertyMap properties = tag->properties();

        songProperties->title = strdup(tag->title().toCString(true));
        songProperties->album = strdup(tag->album().toCString(true));
        songProperties->artist = strdup(tag->artist().toCString(true));

        if(properties.contains("ALBUMARTIST"))
            songProperties->album_artist = strdup(properties["ALBUMARTIST"].toString().toCString(true));
        else if(properties.contains("ALBUM_ARTIST"))
            songProperties->album_artist = strdup(properties["ALBUM_ARTIST"].toString().toCString(true));
        else if(properties.contains("ALBUM ARTIST"))
            songProperties->album_artist = strdup(properties["ALBUM ARTIST"].toString().toCString(true));

        songProperties->year = tag->year();
        songProperties->track_number = tag->track();
        songProperties->duration = audioProperties->length();

        TagLib::Picture picture = tag->pictures()[TagLib::Picture::Type::FrontCover].front();
        TagLib::ByteVector pictureData = picture.data();
        songProperties->picture_data = (char*) malloc(pictureData.size());
        memcpy(songProperties->picture_data, pictureData.data(), pictureData.size());
        songProperties->picture_data_len = pictureData.size();
        songProperties->picture_mime = strdup(picture.mime().toCString(true));

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
