package insights

import (
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/vincer2040/lexi-insights/internal/render"
	"github.com/vincer2040/lexi-insights/internal/routes"
)

func Main() {
	e := echo.New()
	e.Renderer = render.New()
	e.Use(middleware.Logger())
	e.GET("/", routes.RootGet)
	e.Logger.Fatal(e.Start(":5173"))
}
