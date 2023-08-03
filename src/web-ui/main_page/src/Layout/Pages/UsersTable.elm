module Layout.Pages.UsersTable exposing (usersTableShow)

-- import Model exposing (AppletEntry, AppletEntryListStatus(..))

import Element
    exposing
        ( Element
        , centerX
        , column
        , el
        , fill
        , height
        , paddingXY
        , px
        , row
        , spacingXY
        , text
        , width
        )
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font
import Model exposing (Model, Msg(..))
import Themes.Base exposing (getTheme)
import UIC.Buttons exposing (simpleButton)


usersTableShow : Model -> Element.Element Msg
usersTableShow model =
    let
        colorScheme =
            getTheme model.theme
    in
    column
        [ centerX
        , width <| px 300
        , height fill

        -- , centerY
        , Border.width 1
        , Border.rounded 5
        , Border.color colorScheme.simpleButtonBorder
        , Background.color colorScheme.basicPanelContentBackground

        -- , Element.explain Debug.todo
        ]
        [ panelHeader model
        , panelContent model
        ]


panelHeader : Model -> Element Msg
panelHeader model =
    let
        colorScheme =
            getTheme model.theme
    in
    row
        [ width fill
        , height <| px 35
        , Border.widthEach { bottom = 1, left = 0, right = 0, top = 0 }
        , Border.roundEach { bottomLeft = 0, bottomRight = 0, topLeft = 4, topRight = 4 }
        , Font.family [ Font.monospace ]
        , Border.color colorScheme.simpleButtonBorder
        , Background.color colorScheme.basicPanelHeaderBackground
        ]
        [ row [ centerX, width fill, paddingXY 10 0 ]
            [ el [ width fill ] <|
                el [ centerX ] <|
                    text "     Users"
            , simpleButton model "Refresh" <| Just (GetAppletList ())
            ]
        ]


panelContent : Model -> Element Msg
panelContent model =
    let
        colorScheme =
            getTheme model.theme
    in
    column
        [ paddingXY 10 10
        , spacingXY 0 10
        , Background.color colorScheme.basicPanelContentBackground
        , Border.roundEach { bottomLeft = 4, bottomRight = 4, topLeft = 0, topRight = 0 }
        , width fill
        ]
        [ -- el [ centerX, Font.size 24 ] <| text "Your Elm App is working!"
          myTable model
        ]


myTable : Model -> Element Msg
myTable model =
    let
        colorScheme =
            getTheme model.theme

        -- maybeListAppletEntry =
        --     case model.applets_list of
        --         DataAppletEntryListRequestSuccess maybeList ->
        --             maybeList
        --         _ ->
        --             Nothing
        -- tableRows =
        --     case maybeListAppletEntry of
        --         Just listAppletEntry ->
        --             List.map
        --                 (\item -> tableRow model item)
        --                 listAppletEntry
        --         Nothing ->
        --             [ Element.none ]
    in
    column [ width fill, Font.family [ Font.monospace ], Font.size 14, spacingXY 0 5 ] <|
        -- tableRows
        []



-- tableRow : Model -> UserEntry -> Element Msg
-- tableRow model appletEntry =
--     let
--         colorScheme =
--             getTheme model.theme
--         currentId =
--             appletEntry.id
--         isSelectedId =
--             case model.applet_selected of
--                 Just selectedApplet ->
--                     if selectedApplet.id == currentId then
--                         True
--                     else
--                         False
--                 Nothing ->
--                     False
--         filename =
--             appletEntry.filename
--         size =
--             appletEntry.size
--     in
--     row
--         [ width fill
--         , if isSelectedId then
--             Background.color colorScheme.simpleButtonBackground
--           else
--             Background.color colorScheme.basicPanelContentBackground
--         , mouseOver [ Background.color colorScheme.simpleButtonBackgroundMouseOver ]
--         , paddingXY 5 5
--         , pointer
--         -- TODO: Implement by applet_id
--         , Events.onClick <| EditorOpenApplet appletEntry
--         ]
--         [ el [] <| text filename
--         , horizontalSpacer
--         , el [] <|
--             text <|
--                 String.concat
--                     [ String.fromInt <|
--                         size
--                     , " Bytes"
--                     ]
--         ]
