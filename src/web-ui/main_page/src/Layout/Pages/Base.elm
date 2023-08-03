module Layout.Pages.Base exposing (pageRegion)

import Element
    exposing
        ( centerX
        , centerY
        , column
        , el
        , fill
        , height
        , padding
        , paddingXY
        , px
        , rgb255
        , row
        , spacing
        , spacingXY
        , text
        , width
        )
import Element.Background as Background
import Element.Font as Font
import Layout.Pages.Applets.Base exposing (appletsPage)
import Layout.Pages.DepartmentsTable exposing (departmentsTableShow)
import Layout.Pages.PermissionsTable exposing (permissionsTableShow)
import Layout.Pages.StatusesTable exposing (statusesTableShow)
import Layout.Pages.UsersTable exposing (usersTableShow)
import Model exposing (Model, Msg(..), Page(..))
import Themes.Base exposing (getTheme)
import Vendor.MonoIconsSvg exposing (MonoIconName(..), monoIconsSvg)


pageRegion : Model -> Element.Element Msg
pageRegion model =
    let
        colorScheme =
            getTheme model.theme

        pages =
            model.pages
    in
    column
        [ paddingXY 20 5
        , width <| px model.window_size_x
        , height fill

        -- , Border.widthEach { bottom = 0, top = 1, left = 0, right = 0 }
        -- , Border.color colorScheme.simpleButtonBorder
        ]
        [ column
            [ centerX
            , width fill
            , height fill
            ]
            [ case pages.current of
                NoPage ->
                    column
                        [ centerX
                        , centerY
                        , height <| px 300

                        -- , Element.explain Debug.todo
                        , spacingXY 0 40
                        ]
                        [ el [ centerX, Font.size 24 ] <| text "Your Elm App is working!"
                        , el [ centerX, Font.size 18, Font.bold ] <| text "Mono icons SVG"
                        , el [ centerX ] <| iconsPage

                        -- , el [ centerX ] <| mySquare
                        -- , el [ padding 40 ] <| text " "
                        ]

                PageApplets ->
                    appletsPage model

                PageDepartments ->
                    departmentsTableShow model

                PagePermissions ->
                    permissionsTableShow model

                PageStatuses ->
                    statusesTableShow model

                PageUsers ->
                    usersTableShow model
            ]
        ]


iconsPage : Element.Element msg
iconsPage =
    column [ spacing 10, padding 10, Background.color <| rgb255 0xA0 0xA0 0xA0 ]
        [ row []
            [ monoIconsSvg MonoIconAdd 24 24 Nothing
            , monoIconsSvg MonoIconArchive 24 24 Nothing
            , monoIconsSvg MonoIconArrowDown 24 24 Nothing
            , monoIconsSvg MonoIconArrowLeftDown 24 24 Nothing
            , monoIconsSvg MonoIconArrowLeft 24 24 Nothing
            , monoIconsSvg MonoIconArrowLeftUp 24 24 Nothing
            , monoIconsSvg MonoIconArrowRightDown 24 24 Nothing
            , monoIconsSvg MonoIconArrowRight 24 24 Nothing
            , monoIconsSvg MonoIconArrowRightUp 24 24 Nothing
            , monoIconsSvg MonoIconArrowUp 24 24 Nothing
            , monoIconsSvg MonoIconAttachment 24 24 Nothing
            , monoIconsSvg MonoIconBackspace 24 24 Nothing
            , monoIconsSvg MonoIconBan 24 24 Nothing
            , monoIconsSvg MonoIconBarChartAlt 24 24 Nothing
            , monoIconsSvg MonoIconBarChart 24 24 Nothing
            , monoIconsSvg MonoIconBoard 24 24 Nothing
            , monoIconsSvg MonoIconBold 24 24 Nothing
            , monoIconsSvg MonoIconBookmark 24 24 Nothing
            , monoIconsSvg MonoIconBook 24 24 Nothing
            , monoIconsSvg MonoIconCalendar 24 24 Nothing
            , monoIconsSvg MonoIconCall 24 24 Nothing
            , monoIconsSvg MonoIconCamera 24 24 Nothing
            , monoIconsSvg MonoIconCaretDown 24 24 Nothing
            , monoIconsSvg MonoIconCaretLeft 24 24 Nothing
            , monoIconsSvg MonoIconCaretRight 24 24 Nothing
            , monoIconsSvg MonoIconCaretUp 24 24 Nothing
            , monoIconsSvg MonoIconCheck 24 24 Nothing
            , monoIconsSvg MonoIconChevronDoubleDown 24 24 Nothing
            , monoIconsSvg MonoIconChevronDoubleLeft 24 24 Nothing
            , monoIconsSvg MonoIconChevronDoubleRight 24 24 Nothing
            , monoIconsSvg MonoIconChevronDoubleUp 24 24 Nothing
            , monoIconsSvg MonoIconChevronDown 24 24 Nothing
            , monoIconsSvg MonoIconChevronLeft 24 24 Nothing
            , monoIconsSvg MonoIconChevronRight 24 24 Nothing
            , monoIconsSvg MonoIconChevronUp 24 24 Nothing
            , monoIconsSvg MonoIconCircleAdd 24 24 Nothing
            , monoIconsSvg MonoIconCircleArrowDown 24 24 Nothing
            , monoIconsSvg MonoIconCircleArrowLeft 24 24 Nothing
            , monoIconsSvg MonoIconCircleArrowRight 24 24 Nothing
            , monoIconsSvg MonoIconCircleArrowUp 24 24 Nothing
            ]
        , row []
            [ monoIconsSvg MonoIconCircleCheck 24 24 Nothing
            , monoIconsSvg MonoIconCircleError 24 24 Nothing
            , monoIconsSvg MonoIconCircleHelp 24 24 Nothing
            , monoIconsSvg MonoIconCircleInformation 24 24 Nothing
            , monoIconsSvg MonoIconCircleRemove 24 24 Nothing
            , monoIconsSvg MonoIconCircle 24 24 Nothing
            , monoIconsSvg MonoIconCircleWarning 24 24 Nothing
            , monoIconsSvg MonoIconClipboardCheck 24 24 Nothing
            , monoIconsSvg MonoIconClipboardList 24 24 Nothing
            , monoIconsSvg MonoIconClipboard 24 24 Nothing
            , monoIconsSvg MonoIconClock 24 24 Nothing
            , monoIconsSvg MonoIconClose 24 24 Nothing
            , monoIconsSvg MonoIconCloudDownload 24 24 Nothing
            , monoIconsSvg MonoIconCloud 24 24 Nothing
            , monoIconsSvg MonoIconCloudUpload 24 24 Nothing
            , monoIconsSvg MonoIconCloudy 24 24 Nothing
            , monoIconsSvg MonoIconComment 24 24 Nothing
            , monoIconsSvg MonoIconCompass 24 24 Nothing
            , monoIconsSvg MonoIconComputer 24 24 Nothing
            , monoIconsSvg MonoIconCopy 24 24 Nothing
            , monoIconsSvg MonoIconCreditCard 24 24 Nothing
            , monoIconsSvg MonoIconDatabase 24 24 Nothing
            , monoIconsSvg MonoIconDeleteAlt 24 24 Nothing
            , monoIconsSvg MonoIconDelete 24 24 Nothing
            , monoIconsSvg MonoIconDocumentAdd 24 24 Nothing
            , monoIconsSvg MonoIconDocumentCheck 24 24 Nothing
            , monoIconsSvg MonoIconDocumentDownload 24 24 Nothing
            , monoIconsSvg MonoIconDocumentEmpty 24 24 Nothing
            , monoIconsSvg MonoIconDocumentRemove 24 24 Nothing
            , monoIconsSvg MonoIconDocument 24 24 Nothing
            , monoIconsSvg MonoIconDownload 24 24 Nothing
            , monoIconsSvg MonoIconDrag 24 24 Nothing
            , monoIconsSvg MonoIconDrop 24 24 Nothing
            , monoIconsSvg MonoIconEditAlt 24 24 Nothing
            , monoIconsSvg MonoIconEdit 24 24 Nothing
            , monoIconsSvg MonoIconEmail 24 24 Nothing
            , monoIconsSvg MonoIconEnter 24 24 Nothing
            , monoIconsSvg MonoIconExpand 24 24 Nothing
            , monoIconsSvg MonoIconExport 24 24 Nothing
            , monoIconsSvg MonoIconExternalLink 24 24 Nothing
            ]
        , row []
            [ monoIconsSvg MonoIconEyeOff 24 24 Nothing
            , monoIconsSvg MonoIconEye 24 24 Nothing
            , monoIconsSvg MonoIconFavorite 24 24 Nothing
            , monoIconsSvg MonoIconFilter1 24 24 Nothing
            , monoIconsSvg MonoIconFilterAlt 24 24 Nothing
            , monoIconsSvg MonoIconFilter 24 24 Nothing
            , monoIconsSvg MonoIconFlag 24 24 Nothing
            , monoIconsSvg MonoIconFog 24 24 Nothing
            , monoIconsSvg MonoIconFolderAdd 24 24 Nothing
            , monoIconsSvg MonoIconFolderCheck 24 24 Nothing
            , monoIconsSvg MonoIconFolderDownload 24 24 Nothing
            , monoIconsSvg MonoIconFolderRemove 24 24 Nothing
            , monoIconsSvg MonoIconFolder 24 24 Nothing
            , monoIconsSvg MonoIconGrid 24 24 Nothing
            , monoIconsSvg MonoIconHeart 24 24 Nothing
            , monoIconsSvg MonoIconHome 24 24 Nothing
            , monoIconsSvg MonoIconImage 24 24 Nothing
            , monoIconsSvg MonoIconInbox 24 24 Nothing
            , monoIconsSvg MonoIconItalic 24 24 Nothing
            , monoIconsSvg MonoIconLaptop 24 24 Nothing
            , monoIconsSvg MonoIconLayers 24 24 Nothing
            , monoIconsSvg MonoIconLayout 24 24 Nothing
            , monoIconsSvg MonoIconLinkAlt 24 24 Nothing
            , monoIconsSvg MonoIconLink 24 24 Nothing
            , monoIconsSvg MonoIconList 24 24 Nothing
            , monoIconsSvg MonoIconLocation 24 24 Nothing
            , monoIconsSvg MonoIconLock 24 24 Nothing
            , monoIconsSvg MonoIconLogIn 24 24 Nothing
            , monoIconsSvg MonoIconLogOut 24 24 Nothing
            , monoIconsSvg MonoIconMap 24 24 Nothing
            , monoIconsSvg MonoIconMegaphone 24 24 Nothing
            , monoIconsSvg MonoIconMenu 24 24 Nothing
            , monoIconsSvg MonoIconMessageAlt 24 24 Nothing
            , monoIconsSvg MonoIconMessage 24 24 Nothing
            , monoIconsSvg MonoIconMinimize 24 24 Nothing
            , monoIconsSvg MonoIconMobile 24 24 Nothing
            , monoIconsSvg MonoIconMoon 24 24 Nothing
            , monoIconsSvg MonoIconNext 24 24 Nothing
            , monoIconsSvg MonoIconNotificationOff 24 24 Nothing
            , monoIconsSvg MonoIconNotification 24 24 Nothing
            ]
        , row []
            [ monoIconsSvg MonoIconOptionsHorizontal 24 24 Nothing
            , monoIconsSvg MonoIconOptionsVertical 24 24 Nothing
            , monoIconsSvg MonoIconPause 24 24 Nothing
            , monoIconsSvg MonoIconPen 24 24 Nothing
            , monoIconsSvg MonoIconPercentage 24 24 Nothing
            , monoIconsSvg MonoIconPin 24 24 Nothing
            , monoIconsSvg MonoIconPlay 24 24 Nothing
            , monoIconsSvg MonoIconPrevious 24 24 Nothing
            , monoIconsSvg MonoIconPrint 24 24 Nothing
            , monoIconsSvg MonoIconRain 24 24 Nothing
            , monoIconsSvg MonoIconRefresh 24 24 Nothing
            , monoIconsSvg MonoIconRemove 24 24 Nothing
            , monoIconsSvg MonoIconReorderAlt 24 24 Nothing
            , monoIconsSvg MonoIconReorder 24 24 Nothing
            , monoIconsSvg MonoIconRepeat 24 24 Nothing
            , monoIconsSvg MonoIconSave 24 24 Nothing
            , monoIconsSvg MonoIconSearch 24 24 Nothing
            , monoIconsSvg MonoIconSelect 24 24 Nothing
            , monoIconsSvg MonoIconSend 24 24 Nothing
            , monoIconsSvg MonoIconSettings 24 24 Nothing
            , monoIconsSvg MonoIconShare 24 24 Nothing
            , monoIconsSvg MonoIconShoppingCartAdd 24 24 Nothing
            , monoIconsSvg MonoIconShoppingCart 24 24 Nothing
            , monoIconsSvg MonoIconShuffle 24 24 Nothing
            , monoIconsSvg MonoIconSnowflake 24 24 Nothing
            , monoIconsSvg MonoIconSnow 24 24 Nothing
            , monoIconsSvg MonoIconSort 24 24 Nothing
            , monoIconsSvg MonoIconSpeakers 24 24 Nothing
            , monoIconsSvg MonoIconStop 24 24 Nothing
            , monoIconsSvg MonoIconStorm 24 24 Nothing
            , monoIconsSvg MonoIconStrikethrough 24 24 Nothing
            , monoIconsSvg MonoIconSunriseAlt 24 24 Nothing
            , monoIconsSvg MonoIconSunrise 24 24 Nothing
            , monoIconsSvg MonoIconSunset 24 24 Nothing
            , monoIconsSvg MonoIconSun 24 24 Nothing
            , monoIconsSvg MonoIconSwitch 24 24 Nothing
            , monoIconsSvg MonoIconTable 24 24 Nothing
            , monoIconsSvg MonoIconTablet 24 24 Nothing
            , monoIconsSvg MonoIconTag 24 24 Nothing
            , monoIconsSvg MonoIconTemperature 24 24 Nothing
            ]
        , row []
            [ monoIconsSvg MonoIconText 24 24 Nothing
            , monoIconsSvg MonoIconThreeRows 24 24 Nothing
            , monoIconsSvg MonoIconTwoColumns 24 24 Nothing
            , monoIconsSvg MonoIconTwoRows 24 24 Nothing
            , monoIconsSvg MonoIconUnderline 24 24 Nothing
            , monoIconsSvg MonoIconUndo 24 24 Nothing
            , monoIconsSvg MonoIconUnlock 24 24 Nothing
            , monoIconsSvg MonoIconUserAdd 24 24 Nothing
            , monoIconsSvg MonoIconUserCheck 24 24 Nothing
            , monoIconsSvg MonoIconUserRemove 24 24 Nothing
            , monoIconsSvg MonoIconUsers 24 24 Nothing
            , monoIconsSvg MonoIconUser 24 24 Nothing
            , monoIconsSvg MonoIconVolumeOff 24 24 Nothing
            , monoIconsSvg MonoIconVolumeUp 24 24 Nothing
            , monoIconsSvg MonoIconWarning 24 24 Nothing
            , monoIconsSvg MonoIconWebcam 24 24 Nothing
            , monoIconsSvg MonoIconWindow 24 24 Nothing
            , monoIconsSvg MonoIconWind 24 24 Nothing
            , monoIconsSvg MonoIconZoomIn 24 24 Nothing
            , monoIconsSvg MonoIconZoomOut 24 24 Nothing
            ]
        ]
