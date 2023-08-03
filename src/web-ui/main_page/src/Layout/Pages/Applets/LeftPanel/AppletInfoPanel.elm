module Layout.Pages.Applets.LeftPanel.AppletInfoPanel exposing (appletInfoPanel)

import Element
    exposing
        ( above
        , centerX
        , centerY
        , column
        , el
        , fill
        , height
        , paddingXY
        , pointer
        , px
        , row
        , spacingXY
        , text
        , width
        )
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font
import Model exposing (AppletArtifact, AppletArtifactStatus(..), Model, Msg(..))
import Themes.Base exposing (getTheme)
import Time
import UIC.Tooltip exposing (myTooltip, tooltip)
import Vendor.Helpers exposing (humanReadableBytes, monthToInt, paddedStringFromInt)


appletInfoPanel : Model -> Element.Element Msg
appletInfoPanel model =
    let
        colorScheme =
            getTheme model.theme

        maybeCurrentApplet =
            case model.pages.applets.artifact of
                DataAppletArtifactRequestSuccess appletArtifact ->
                    Just appletArtifact

                _ ->
                    Nothing
    in
    case maybeCurrentApplet of
        Just currentApplet ->
            let
                appletOidReduced =
                    String.concat
                        [ String.slice 0 8 currentApplet.oid
                        , "..."
                        ]
            in
            column
                [ centerX
                , width fill
                , height <| px 150
                , Border.width 1
                , Border.rounded 5
                , Border.color colorScheme.simpleButtonBorder
                , Background.color colorScheme.basicPanelContentBackground
                , Font.family [ Font.monospace ]

                -- , Element.explain Debug.todo
                ]
                [ -- Panel Header
                  row
                    [ width fill
                    , height <| px 35

                    -- , Border.width 1
                    , Border.widthEach { bottom = 1, left = 0, right = 0, top = 0 }
                    , Border.roundEach { bottomLeft = 0, bottomRight = 0, topLeft = 4, topRight = 4 }
                    , Font.family [ Font.monospace ]

                    -- , Border.rounded 5
                    , Border.color colorScheme.simpleButtonBorder
                    , Background.color colorScheme.basicPanelHeaderBackground
                    ]
                    [ row [ width fill, paddingXY 10 10 ]
                        [ el
                            [ centerX ]
                          <|
                            text "File info"
                        ]
                    ]
                , column
                    [ spacingXY 0 10
                    , centerX
                    , centerY
                    , width <| px 305
                    , height <| px 90
                    , Background.color colorScheme.basicPanelContentBackground
                    , Border.roundEach { bottomLeft = 4, bottomRight = 4, topLeft = 0, topRight = 0 }
                    ]
                    [ row [ width fill ]
                        [ row [ width fill ]
                            [ el [ Font.bold ] <| text "Id: "
                            , el [ tooltip above (myTooltip model currentApplet.oid) ] <| text appletOidReduced
                            ]
                        , row
                            [ width fill ]
                            [ el [ Font.bold ] <| text "Size: "
                            , el [] <|
                                text <|
                                    humanReadableBytes <|
                                        Just currentApplet.size
                            ]
                        ]
                    , row
                        []
                        [ el [ Font.bold ] <| text "Filename: "
                        , el [] <| text currentApplet.filename
                        ]
                    , row
                        []
                        [ el [ Font.bold ] <| text "Created at: "
                        , case model.timezone of
                            Just timeZone ->
                                el [] <|
                                    let
                                        createdAtPosix =
                                            Time.millisToPosix <| currentApplet.created_at * 1000

                                        hour =
                                            paddedStringFromInt <| Time.toHour timeZone createdAtPosix

                                        minute =
                                            paddedStringFromInt <| Time.toMinute timeZone createdAtPosix

                                        second =
                                            paddedStringFromInt <| Time.toSecond timeZone createdAtPosix

                                        year =
                                            String.fromInt <| Time.toYear timeZone createdAtPosix

                                        month =
                                            paddedStringFromInt <| monthToInt <| Time.toMonth timeZone createdAtPosix

                                        day =
                                            paddedStringFromInt <| Time.toDay timeZone createdAtPosix
                                    in
                                    text <|
                                        String.concat
                                            [ year
                                            , "-"
                                            , month
                                            , "-"
                                            , day
                                            , " "
                                            , hour
                                            , ":"
                                            , minute
                                            , ":"
                                            , second
                                            ]

                            Nothing ->
                                Element.none
                        ]
                    , row
                        []
                        [ el [ Font.bold ] <| text "Updated at: "
                        , case model.timezone of
                            Just timeZone ->
                                el [] <|
                                    let
                                        updatedAtPosix =
                                            Time.millisToPosix <| currentApplet.updated_at * 1000

                                        hour =
                                            paddedStringFromInt <| Time.toHour timeZone updatedAtPosix

                                        minute =
                                            paddedStringFromInt <| Time.toMinute timeZone updatedAtPosix

                                        second =
                                            paddedStringFromInt <| Time.toSecond timeZone updatedAtPosix

                                        year =
                                            String.fromInt <| Time.toYear timeZone updatedAtPosix

                                        month =
                                            paddedStringFromInt <| monthToInt <| Time.toMonth timeZone updatedAtPosix

                                        day =
                                            paddedStringFromInt <| Time.toDay timeZone updatedAtPosix
                                    in
                                    text <|
                                        String.concat
                                            [ year
                                            , "-"
                                            , month
                                            , "-"
                                            , day
                                            , " "
                                            , hour
                                            , ":"
                                            , minute
                                            , ":"
                                            , second
                                            ]

                            Nothing ->
                                Element.none
                        ]
                    ]
                ]

        Nothing ->
            Element.none
