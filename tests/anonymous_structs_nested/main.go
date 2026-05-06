package main

import "fmt"

// Named struct with anonymous struct field
type Config struct {
	Name     string
	Database struct {
		Host        string
		Port        int
		Credentials struct {
			User     string
			Password string
		}
	}
	Cache struct {
		Enabled bool
		TTL     int
	}
}

// Named struct with slice of anonymous structs
type Dashboard struct {
	Title   string
	Widgets []struct {
		ID       int
		Type     string
		Position struct {
			X, Y int
		}
	}
}

func main() {
	// Test named struct with anonymous struct field
	config := Config{
		Name: "production",
	}
	config.Database.Host = "db.example.com"
	config.Database.Port = 5432
	config.Database.Credentials.User = "admin"
	config.Database.Credentials.Password = "secret"
	config.Cache.Enabled = true
	config.Cache.TTL = 300

	fmt.Printf("Config: %s\n", config.Name)
	fmt.Printf("Database: %s:%d (user: %s)\n",
		config.Database.Host, config.Database.Port, config.Database.Credentials.User)
	fmt.Printf("Cache: enabled=%v, TTL=%d\n", config.Cache.Enabled, config.Cache.TTL)

	// Test named struct with slice of anonymous structs
	dashboard := Dashboard{
		Title: "Main Dashboard",
		Widgets: []struct {
			ID       int
			Type     string
			Position struct {
				X, Y int
			}
		}{
			{
				ID:       1,
				Type:     "chart",
				Position: struct{ X, Y int }{X: 0, Y: 0},
			},
			{
				ID:       2,
				Type:     "table",
				Position: struct{ X, Y int }{X: 100, Y: 0},
			},
		},
	}

	fmt.Printf("\nDashboard: %s\n", dashboard.Title)
	for _, widget := range dashboard.Widgets {
		fmt.Printf("Widget %d (%s) at position (%d, %d)\n",
			widget.ID, widget.Type, widget.Position.X, widget.Position.Y)
	}

	// Deeply nested anonymous structs
	var system struct {
		Version string
		Modules map[string]struct {
			Enabled  bool
			Settings struct {
				Options []struct {
					Key   string
					Value interface{}
				}
			}
		}
	}

	system.Version = "1.0.0"
	system.Modules = make(map[string]struct {
		Enabled  bool
		Settings struct {
			Options []struct {
				Key   string
				Value interface{}
			}
		}
	})

	// Add a module with settings
	authModule := struct {
		Enabled  bool
		Settings struct {
			Options []struct {
				Key   string
				Value interface{}
			}
		}
	}{
		Enabled: true,
	}
	authModule.Settings.Options = []struct {
		Key   string
		Value interface{}
	}{
		{Key: "timeout", Value: 3600},
		{Key: "max_attempts", Value: 3},
	}
	system.Modules["auth"] = authModule

	fmt.Printf("\nSystem version: %s\n", system.Version)
	for name, module := range system.Modules {
		fmt.Printf("Module %s: enabled=%v\n", name, module.Enabled)
		for _, opt := range module.Settings.Options {
			fmt.Printf("  - %s: %v\n", opt.Key, opt.Value)
		}
	}
}
