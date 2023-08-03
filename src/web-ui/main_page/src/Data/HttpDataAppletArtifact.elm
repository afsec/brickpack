module Data.HttpDataAppletArtifact exposing (jsonDecodeAppletArtifact)

import Json.Decode exposing (Decoder, decodeString, field, int, string, succeed)
import Json.Decode.Extra exposing (andMap)
import Model exposing (AppletArtifact)


jsonDecodeAppletArtifact : String -> Result Json.Decode.Error AppletArtifact
jsonDecodeAppletArtifact jsonData =
    decodeString appletEntryDecoder jsonData


appletEntryDecoder : Decoder AppletArtifact
appletEntryDecoder =
    succeed AppletArtifact
        |> andMap (field "oid" string)
        |> andMap (field "filename" string)
        |> andMap (field "code" string)
        |> andMap (field "size" int)
        |> andMap (field "created_at" int)
        |> andMap (field "updated_at" int)
