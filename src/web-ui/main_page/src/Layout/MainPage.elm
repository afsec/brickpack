module Layout.MainPage exposing (mainPageLayout)

import Element
    exposing
        ( column
        , fill
        , height
        , width
        )
import Layout.Header.Base exposing (headerRegion)
import Layout.Pages.Base exposing (pageRegion)
import Model exposing (Model, Msg(..))
import Themes.Base exposing (getTheme)


mainPageLayout : Model -> Element.Element Msg
mainPageLayout model =
    let
        colorScheme =
            getTheme model.theme
    in
    column
        [ width fill
        , height fill
        ]
        [ headerRegion model
        , pageRegion model
        ]
