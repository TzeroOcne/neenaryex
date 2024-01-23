<script lang="ts">
  import { page } from '$app/stores';
  import { showShidebar as show } from '$lib/store/app.store';
  import { Button, Sidebar, SidebarGroup, SidebarItem, SidebarWrapper } from 'flowbite-svelte';
  import { Icon } from 'flowbite-svelte-icons';
  import './sidebar.css';

  const navList:{
    path: string,
    label:string,
    iconName:string,
  }[] = [
    {
      path: '/',
      label: 'Home',
      iconName: 'home-outline',
    },
    {
      path: '/profile',
      label: 'Profile',
      iconName: 'profile-card-outline',
    },
  ];
  
  const setShow = (val:boolean) => {
    show.set(val);
  };
  
  const toggleShow = () => {
    setShow(!$show);
  };
  
  $: activeUrl = $page.url.pathname;
</script>

<div id="sidebar-root" class:show={$show} class="dark:text-white">
  <Sidebar class="w-full h-full">
    <SidebarWrapper id="sidebar-wrapper" class="rounded-none h-full dark:bg-nnrygray-800">
      <SidebarGroup id="sidebar-group">
        <div id="button-container" class:show={$show}>
          <Button id="toggle-menu" class="p-0 m-0 w-8 h-8 relative bg-nnry-800"
            on:click={toggleShow}
          >
            <div id="button-icon" class:show={$show}>
              <Icon name="chervon-double-left-solid" class="inline"/>
            </div>
            <div id="button-icon" class="absolute" class:show={!$show}>
              <Icon name="chervon-double-right-solid" class="inline"/>
            </div>
          </Button>
        </div>
        {#each navList as { path, label, iconName } }
        <SidebarItem id="sidebar-item" label={label} href={path} active={ activeUrl.replace(/(.)\/$/, '$1') === path }>
          <svelte:fragment slot="icon">
            <div id="sidebar-icon">
              <Icon name={iconName} />
            </div>
          </svelte:fragment>
        </SidebarItem>
        {/each}
      </SidebarGroup>
    </SidebarWrapper>
  </Sidebar>
</div>

<style lang="postcss">
  div#button-container {
    /* transition: left .5s ease, transform .5s ease; */
    @apply text-right;
  }

  /* div#button-container.show {
    @apply left-full -translate-x-full;
  } */

  div#button-icon {
    transition: opacity .5s ease;
    @apply w-full h-full flex items-center justify-around opacity-0;
  }
  
  div#button-icon.show {
    opacity: 1;
  }
  
  div#sidebar-icon {
    @apply m-0 p-0 w-[32px] h-[32px] flex items-center justify-around absolute;
  }
</style>
