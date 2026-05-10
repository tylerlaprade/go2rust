package core

import "context"

type Exporter func(context.Context) context.Context

func Use(ctx context.Context, exporter Exporter) context.Context {
	return exporter(ctx)
}
