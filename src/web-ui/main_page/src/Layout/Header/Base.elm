module Layout.Header.Base exposing (headerRegion)

import Element
    exposing
        ( fill
        , height
        , paddingEach
        , px
        , row
        , scrollbarX
        , spacingXY
        , width
        )
import Layout.Header.RepoPanel exposing (repoPanel)
import Model exposing (Model, Msg(..), Page(..), ResultButtonState(..))
import Themes.Base exposing (getTheme)
import UIC.Buttons exposing (simpleButton)
import UIC.ResultButtons exposing (resultButton)


headerRegion : Model -> Element.Element Msg
headerRegion model =
    let
        colorScheme =
            getTheme model.theme

        btnState =
            model.header_panel.refresh_btn
    in
    row
        [ width fill
        , height <| px 100
        , spacingXY 40 0
        , scrollbarX
        , paddingEach
            { top = 8
            , right = 0
            , bottom = 10
            , left = 0
            }
        ]
        [ resultButton model btnState "Refresh" <| Just MainPage
        , repoPanel model PageUsers "Users" model.header_panel.users
        , repoPanel model PageDepartments "Departments" model.header_panel.departments
        , repoPanel model PagePermissions "Permissions" model.header_panel.permissions
        , repoPanel model PageStatuses "Statuses" model.header_panel.statuses
        , repoPanel model PageApplets "Applets" model.header_panel.applets
        , simpleButton model "toggle theme" <| Just ToggleTheme
        , simpleButton model "Logout" <| Just EndSession
        ]
