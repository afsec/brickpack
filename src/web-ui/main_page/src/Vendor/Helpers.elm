module Vendor.Helpers exposing (elementAtrributeNone, expectStringDetailed, httpErrorToString, humanReadableBytes, monthToInt, paddedStringFromInt, sortAppletEntryListByfilename)

import Element
import Filesize
import Http
import Model exposing (AppletEntry)
import NaturalOrdering
import Time exposing (Month(..))


{-| elementAtrributeNone
Is a "hack" to use as a sort of "Element.Attribute.none".
Because basically "everything" is alpha 1.
-}
elementAtrributeNone : Element.Attr decorative msg
elementAtrributeNone =
    Element.alpha 1


sortAppletEntryListByfilename : List AppletEntry -> List AppletEntry
sortAppletEntryListByfilename list =
    List.sortWith sortAppletEntryListByfilenameInner
        list


sortAppletEntryListByfilenameInner : AppletEntry -> AppletEntry -> Order
sortAppletEntryListByfilenameInner a b =
    NaturalOrdering.compare a.filename b.filename



--
-- Reference: https://jzxhuang.medium.com/going-beyond-200-ok-a-guide-to-detailed-http-responses-in-elm-6ddd02322e


expectStringDetailed : (Result Http.Error ( Http.Metadata, String ) -> msg) -> Http.Expect msg
expectStringDetailed msg =
    Http.expectStringResponse msg convertResponseString


convertResponseString : Http.Response String -> Result Http.Error ( Http.Metadata, String )
convertResponseString httpResponse =
    case httpResponse of
        Http.BadUrl_ url ->
            Err (Http.BadUrl url)

        Http.Timeout_ ->
            Err Http.Timeout

        Http.NetworkError_ ->
            Err Http.NetworkError

        Http.BadStatus_ metadata _ ->
            Err (Http.BadStatus metadata.statusCode)

        Http.GoodStatus_ metadata body ->
            Ok ( metadata, body )



--


httpErrorToString : Http.Error -> String
httpErrorToString httpError =
    case httpError of
        Http.BadUrl url ->
            "Http.BadUrl: [" ++ url ++ "]"

        Http.Timeout ->
            "Http.Timeout"

        Http.NetworkError ->
            "Http.NetworkError"

        Http.BadStatus statusCode ->
            "Http.BadStatus: [" ++ String.fromInt statusCode ++ "]"

        Http.BadBody body ->
            "Http.BadBody: [" ++ body ++ "]"


humanReadableBytes : Maybe Int -> String
humanReadableBytes maybeBytes =
    case maybeBytes of
        Just bytes ->
            Filesize.formatBase2 bytes

        Nothing ->
            Filesize.formatBase2 0


paddedStringFromInt : Int -> String
paddedStringFromInt number =
    if number < 10 then
        "0" ++ String.fromInt number

    else
        String.fromInt number


monthToInt : Month -> Int
monthToInt month =
    case month of
        Jan ->
            1

        Feb ->
            2

        Mar ->
            3

        Apr ->
            4

        May ->
            5

        Jun ->
            6

        Jul ->
            7

        Aug ->
            8

        Sep ->
            9

        Oct ->
            10

        Nov ->
            11

        Dec ->
            12
