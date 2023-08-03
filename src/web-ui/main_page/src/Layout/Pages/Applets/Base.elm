module Layout.Pages.Applets.Base exposing (appletsPage)

import Element
    exposing
        ( Element
        , fill
        , height
        , px
        , row
        , spacingXY
        , width
        )
import Layout.Pages.Applets.EditorPanel exposing (appletsEditorPanel)
import Layout.Pages.Applets.LeftPanel.Base exposing (appletsLeftPanel)
import Model exposing (AppletEntryListStatus(..), Model, Msg(..))


appletsPage : Model -> Element Msg
appletsPage model =
    row
        [ width <| px (model.window_size_x - 50)
        , height <| px 500
        , spacingXY 15 0

        -- , Element.explain Debug.todo
        ]
        [ appletsLeftPanel model
        , appletsEditorPanel model
        ]
