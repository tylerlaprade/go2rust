package event

import (
	"context"

	"example.com/contextshared/core"
)

type Exporter func(context.Context) context.Context

func SetExporter(exporter Exporter) context.Context {
	return core.Use(context.Background(), core.Exporter(exporter))
}
