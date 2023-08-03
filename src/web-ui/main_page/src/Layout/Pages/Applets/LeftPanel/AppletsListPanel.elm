module Layout.Pages.Applets.LeftPanel.AppletsListPanel exposing (appletsListPanel)

import Element
    exposing
        ( Element
        , centerX
        , column
        , el
        , fill
        , height
        , mouseOver
        , paddingXY
        , pointer
        , px
        , row
        , scrollbarY
        , spacingXY
        , text
        , width
        )
import Element.Background as Background
import Element.Border as Border
import Element.Events as Events
import Element.Font as Font
import Model exposing (AppletEntry, AppletEntryListStatus(..), Model, Msg(..))
import Themes.Base exposing (getTheme)
import UIC.Buttons exposing (toolbarButton)
import UIC.Misc exposing (horizontalSpacer)
import Vendor.Helpers exposing (humanReadableBytes)
import Vendor.MonoIconsSvg exposing (MonoIconName(..))


appletsListPanel : Model -> Element Msg
appletsListPanel model =
    let
        colorScheme =
            getTheme model.theme
    in
    column
        [ width fill

        -- width <| px 300
        -- , height <| px 350
        , height fill
        , centerX
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

        pages =
            model.pages

        applets =
            pages.applets
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
            [ row [ spacingXY 10 0 ]
                [ toolbarButton model Nothing MonoIconRefresh <| Just RefreshAppletList
                ]
            , el [ width fill ] <|
                el
                    [ centerX ]
                <|
                    text "Applets"

            -- , horizontalSpacer
            , row [ spacingXY 10 0 ]
                [ toolbarButton model Nothing MonoIconAdd <| Just EditorNewApplet
                ]

            -- , simpleButton model "Refresh" <| Just (GetAppletList ())
            ]
        ]


panelContent : Model -> Element Msg
panelContent model =
    let
        colorScheme =
            getTheme model.theme
    in
    column
        [ width fill
        , paddingXY 5 5

        -- paddingXY 10 10
        -- , spacingXY 0 10
        , height fill
        , Background.color colorScheme.basicPanelContentBackground
        , Border.roundEach { bottomLeft = 4, bottomRight = 4, topLeft = 0, topRight = 0 }
        ]
        [ -- el [ centerX, Font.size 24 ] <| text "Your Elm App is working!"
          myTable model
        ]


myTable : Model -> Element Msg
myTable model =
    let
        colorScheme =
            getTheme model.theme

        pages =
            model.pages

        applets =
            pages.applets

        maybeListAppletEntry =
            case applets.list of
                DataAppletEntryListRequestSuccess appletEntryList ->
                    Just appletEntryList

                _ ->
                    Nothing

        tableRows =
            case maybeListAppletEntry of
                Just listAppletEntry ->
                    List.map
                        (\item -> tableRow model item)
                        listAppletEntry

                Nothing ->
                    [ Element.none ]
    in
    column
        [ width fill

        -- , height <| px 312
        , height fill

        -- , height fill
        , Font.family [ Font.monospace ]
        , Font.size 14
        , spacingXY 0 5
        , scrollbarY
        ]
    <|
        tableRows


tableRow : Model -> AppletEntry -> Element Msg
tableRow model appletEntry =
    let
        colorScheme =
            getTheme model.theme

        pages =
            model.pages

        applets =
            pages.applets

        currentId =
            appletEntry.oid

        isSelectedId =
            case applets.selected of
                Just selectedApplet ->
                    if selectedApplet.oid == currentId then
                        True

                    else
                        False

                Nothing ->
                    False

        filename =
            appletEntry.filename

        size =
            appletEntry.size
    in
    row
        [ width fill
        , Border.rounded 5
        , if isSelectedId then
            Background.color colorScheme.simpleButtonBackground

          else
            Background.color colorScheme.basicPanelContentBackground
        , mouseOver [ Background.color colorScheme.simpleButtonBackgroundMouseOver ]
        , paddingXY 5 5
        , pointer

        -- TODO: Implement by applet_id
        , Events.onClick <| EditorOpenApplet appletEntry
        ]
        [ el [] <| text filename
        , horizontalSpacer
        , el [] <|
            text <|
                humanReadableBytes <|
                    Just size
        ]
