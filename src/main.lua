
function update( msg )
    print( msg )
    if msg=='Exit Game' then
        return true
    end
end

local function window( theme )
    local palette = theme:palette()
    local palext = theme:extended_palette()

    return iced.Container.style()
    :border( iced.border( palext.background.strong.color, 1, 10 ) )
    :background( palette.background )
end

function view()
    return iced.container(
        iced.container(
            iced.column{
                iced.button('Load Game'),
                iced.button('New Game'):on_press('New Game'),
                iced.button('Editors'):on_press('Editors'),
                iced.button('Options'):on_press('Options'),
                iced.button('Credits'):on_press('Credits'),
                iced.button('Exit Game'):on_press('Exit Game'),
            }
            :spacing(10)
            :padding(20)
            :align_x( iced.Center() )
        )
        :style( window )
        :align_x( iced.Center() )
        :width( 150 )
    )
    :center( iced.Fill() )
end

function main ()
   print("Hello")
   iced.run( update, view )
   print("done")
end
