pub mod color_schemes;
pub mod icon_sets;

#[cfg(test)]
mod tests {
    use crate::config::theme::UserTheme;
    use crate::config::types::ComponentId;

    #[test]
    fn test_all_color_schemes_load() {
        let schemes = super::color_schemes::all();
        assert!(schemes.len() >= 10, "expected at least 10 color schemes");
        for scheme in &schemes {
            assert!(!scheme.name.is_empty());
            // Every scheme should define colors for all non-separator components
            for id in ComponentId::ALL {
                assert!(
                    scheme.get(*id).is_some(),
                    "color scheme '{}' missing component {:?}",
                    scheme.name,
                    id
                );
            }
        }
    }

    #[test]
    fn worktree_inherits_each_palette_git_status_colors() {
        for scheme in super::color_schemes::all() {
            let worktree = scheme.get(ComponentId::Worktree).unwrap();
            let git = scheme.get(ComponentId::Git).unwrap();

            assert_eq!(worktree.icon, git.icon, "{} icon color", scheme.name);
            assert_eq!(worktree.text, git.text, "{} text color", scheme.name);
            assert_eq!(
                worktree.background, git.background,
                "{} background color",
                scheme.name
            );
            assert_eq!(
                worktree.text_bold, git.text_bold,
                "{} bold style",
                scheme.name
            );
        }
    }

    #[test]
    fn default_palette_visually_separates_directory_and_worktree() {
        let scheme = super::color_schemes::find("Default").unwrap();
        let directory = scheme.get(ComponentId::Directory).unwrap();
        let worktree = scheme.get(ComponentId::Worktree).unwrap();

        assert!(
            directory.icon != worktree.icon
                || directory.text != worktree.text
                || directory.background != worktree.background
        );
    }

    #[test]
    fn test_all_icon_sets_load() {
        let sets = super::icon_sets::all();
        assert!(sets.len() >= 5, "expected at least 5 icon sets");
        for set in &sets {
            assert!(!set.name.is_empty());
            for id in ComponentId::ALL {
                assert!(
                    set.get(*id).is_some(),
                    "icon set '{}' missing component {:?}",
                    set.name,
                    id
                );
            }
        }
    }

    #[test]
    fn test_apply_color_scheme() {
        let mut theme = UserTheme::default_theme();
        let nord = super::color_schemes::find("Nord").unwrap();
        nord.apply_to(&mut theme.components);

        // Nord uses RGB backgrounds
        let model = theme.get_component(ComponentId::Model).unwrap();
        assert!(model.colors.background.is_some());
    }

    #[test]
    fn test_apply_icon_set() {
        let mut theme = UserTheme::default_theme();
        let powerline = super::icon_sets::find("Powerline").unwrap();
        powerline.apply_to(&mut theme.components);

        // Powerline separator should provide both the plain and Nerd Font connectors.
        let sep = theme.get_component(ComponentId::Separator).unwrap();
        assert_eq!(sep.icon.plain, "\u{25ba}");
        assert!(sep.icon.nerd_font.contains('\u{e0b0}'));
    }

    #[test]
    fn test_color_scheme_does_not_change_icons() {
        let mut theme = UserTheme::default_theme();
        let original_icon = theme
            .get_component(ComponentId::Model)
            .unwrap()
            .icon
            .plain
            .clone();

        let gruvbox = super::color_schemes::find("Gruvbox").unwrap();
        gruvbox.apply_to(&mut theme.components);

        let after_icon = theme
            .get_component(ComponentId::Model)
            .unwrap()
            .icon
            .plain
            .clone();
        assert_eq!(
            original_icon, after_icon,
            "color scheme should not change icons"
        );
    }

    #[test]
    fn test_icon_set_does_not_change_colors() {
        let mut theme = UserTheme::default_theme();
        let original_color = theme
            .get_component(ComponentId::Model)
            .unwrap()
            .colors
            .icon
            .clone();

        let minimal = super::icon_sets::find("Minimal").unwrap();
        minimal.apply_to(&mut theme.components);

        let after_color = theme
            .get_component(ComponentId::Model)
            .unwrap()
            .colors
            .icon
            .clone();
        assert_eq!(
            original_color, after_color,
            "icon set should not change colors"
        );
    }

    #[test]
    fn test_find_case_insensitive() {
        assert!(super::color_schemes::find("nord").is_some());
        assert!(super::color_schemes::find("NORD").is_some());
        assert!(super::color_schemes::find("Nord").is_some());
        assert!(super::icon_sets::find("emoji").is_some());
        assert!(super::icon_sets::find("EMOJI").is_some());
    }
}
