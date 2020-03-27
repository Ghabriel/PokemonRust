macro_rules! assert_event {
    ($value:expr, InitialSwitchIn { $($args:tt)* }) => {
        assert_pattern!($value, BattleEvent::InitialSwitchIn(InitialSwitchIn { $($args)* }));
    };
    ($value:expr, ChangeTurn { $($args:tt)* }) => {
        assert_pattern!($value, BattleEvent::ChangeTurn(ChangeTurn { $($args)* }));
    };
    ($value:expr, Damage { $($args:tt)* }) => {
        assert_pattern!($value, BattleEvent::Damage(Damage { $($args)* }));
    };
    ($value:expr, Miss { $($args:tt)* }) => {
        assert_pattern!($value, BattleEvent::Miss(Miss { $($args)* }));
    };
    ($value:expr, StatChange { $($args:tt)* }) => {
        assert_pattern!($value, BattleEvent::StatChange(StatChange { $($args)* }));
    };
    ($value:expr, Faint { $($args:tt)* }) => {
        assert_pattern!($value, BattleEvent::Faint(Faint { $($args)* }));
    };
}

macro_rules! assert_pattern {
    ($value:expr, $pattern:pat) => {
        match $value {
            $pattern => {},
            _ => panic!(
                "\nPattern mismatch.\n\tValue: {:?}\n\tPattern: {}\n",
                $value,
                stringify!($pattern),
            ),
        }
    };
}

macro_rules! battle {
    ($($args:tt)*) => {
        {
            let mut backend = battle_setup!($($args)*);
            let _ = backend.tick();
            backend
        }
    }
}

macro_rules! battle_setup {
    (
        $p1_species:literal $p1_level:literal $(($($p1_data:tt)*))?
        vs
        $p2_species:literal $p2_level:literal $(($($p2_data:tt)*))?
    ) => {
        {
            let p1 = pokemon_setup!($p1_species $p1_level $(($($p1_data)*))?);
            let p2 = pokemon_setup!($p2_species $p2_level $(($($p2_data)*))?);

            create_simple_wild_battle(p1, p2)
        }
    }
}

macro_rules! pokemon_setup {
    ($p1_species:literal $p1_level:literal $(($($p1_data:tt)*))?) => {
        {
            let p1_builder = PokemonBuilder::default();
            $(let p1_builder = constrain_pokemon!(p1_builder, $($p1_data)*);)?

            let pokedex = get_all_pokemon_species();
            let movedex = get_all_moves();

            p1_builder.build(
                &pokedex.get_species($p1_species).unwrap(),
                &movedex,
                $p1_level,
            )
        }
    }
}

macro_rules! constrain_pokemon {
    ($builder:ident, max ivs$(, $($data:tt)*)*) => {
        {
            let $builder = $builder.with_ivs([31; 6]);
            constrain_pokemon!($builder, $($($data)?)*)
        }
    };

    ($builder:ident, Hardy) => {
        $builder.with_nature(Nature::Hardy)
    };

    ($builder:ident, Lonely) => {
        $builder.with_nature(Nature::Lonely)
    };

    ($builder:ident, Adamant) => {
        $builder.with_nature(Nature::Adamant)
    };

    ($builder:ident, Naughty) => {
        $builder.with_nature(Nature::Naughty)
    };

    ($builder:ident, Brave) => {
        $builder.with_nature(Nature::Brave)
    };

    ($builder:ident, Bold) => {
        $builder.with_nature(Nature::Bold)
    };

    ($builder:ident, Docile) => {
        $builder.with_nature(Nature::Docile)
    };

    ($builder:ident, Impish) => {
        $builder.with_nature(Nature::Impish)
    };

    ($builder:ident, Lax) => {
        $builder.with_nature(Nature::Lax)
    };

    ($builder:ident, Relaxed) => {
        $builder.with_nature(Nature::Relaxed)
    };

    ($builder:ident, Modest) => {
        $builder.with_nature(Nature::Modest)
    };

    ($builder:ident, Mild) => {
        $builder.with_nature(Nature::Mild)
    };

    ($builder:ident, Bashful) => {
        $builder.with_nature(Nature::Bashful)
    };

    ($builder:ident, Rash) => {
        $builder.with_nature(Nature::Rash)
    };

    ($builder:ident, Quiet) => {
        $builder.with_nature(Nature::Quiet)
    };

    ($builder:ident, Calm) => {
        $builder.with_nature(Nature::Calm)
    };

    ($builder:ident, Gentle) => {
        $builder.with_nature(Nature::Gentle)
    };

    ($builder:ident, Careful) => {
        $builder.with_nature(Nature::Careful)
    };

    ($builder:ident, Quirky) => {
        $builder.with_nature(Nature::Quirky)
    };

    ($builder:ident, Sassy) => {
        $builder.with_nature(Nature::Sassy)
    };

    ($builder:ident, Timid) => {
        $builder.with_nature(Nature::Timid)
    };

    ($builder:ident, Hasty) => {
        $builder.with_nature(Nature::Hasty)
    };

    ($builder:ident, Jolly) => {
        $builder.with_nature(Nature::Jolly)
    };

    ($builder:ident, Naive) => {
        $builder.with_nature(Nature::Naive)
    };

    ($builder:ident, Serious) => {
        $builder.with_nature(Nature::Serious)
    };
}
