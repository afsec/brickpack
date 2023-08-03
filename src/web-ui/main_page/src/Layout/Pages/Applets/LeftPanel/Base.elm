module Layout.Pages.Applets.LeftPanel.Base exposing (appletsLeftPanel)

import Element
    exposing
        ( Element
        , column
        , fill
        , height
        , px
        , row
        , spacingXY
        , width
        )
import Layout.Pages.Applets.LeftPanel.AppletInfoPanel exposing (appletInfoPanel)
import Layout.Pages.Applets.LeftPanel.AppletsListPanel exposing (appletsListPanel)
import Model exposing (AppletEntryListStatus(..), Model, Msg(..))


appletsLeftPanel : Model -> Element Msg
appletsLeftPanel model =
    column
        [ width <| px 320
        , height fill
        , spacingXY 0 15
        ]
        [ appletsListPanel model
        , appletInfoPanel model
        ]
