import React from 'react';
import { AppShell, Text, Burger, useMantineTheme } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { Link, Outlet } from 'react-router-dom';

export function Layout() {
  const theme = useMantineTheme();
  const [opened, { toggle }] = useDisclosure(false);

  return (
    <AppShell
      padding="md"
      header={{ height: 60 }}
      navbar={{
        width: 300,
        breakpoint: 'sm',
        collapsed: { mobile: !opened },
      }}
    >
      <AppShell.Header>
        <Burger opened={opened} onClick={toggle} hiddenFrom="sm" size="sm" />
        <div style={{ display: 'flex', alignItems: 'center', height: '100%' }}>
          <Text>Isonomia Voting System</Text>
        </div>
      </AppShell.Header>

      <AppShell.Navbar p="md">
        <Text component={Link} to="/">Home</Text>
        <Text component={Link} to="/vote">Vote</Text>
        <Text component={Link} to="/results">Results</Text>
      </AppShell.Navbar>

      <AppShell.Main>
        <Outlet />
      </AppShell.Main>
    </AppShell>
  );
}
