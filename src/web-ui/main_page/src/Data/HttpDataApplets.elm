module Data.HttpDataApplets exposing (jsonDecodeAppletEntryList)

import Json.Decode exposing (Decoder, decodeString, field, int, list, string, succeed)
import Json.Decode.Extra exposing (andMap)
import Model exposing (AppletEntry)


jsonDecodeAppletEntryList : String -> Result Json.Decode.Error (List AppletEntry)
jsonDecodeAppletEntryList jsonData =
    decodeString appletEntryListDecoder jsonData


appletEntryListDecoder : Decoder (List AppletEntry)
appletEntryListDecoder =
    list appletEntryDecoder


appletEntryDecoder : Decoder AppletEntry
appletEntryDecoder =
    succeed AppletEntry
        |> andMap (field "oid" string)
        |> andMap (field "filename" string)
        |> andMap (field "size" int)
        |> andMap (field "created_at" int)
        |> andMap (field "updated_at" int)
