<template>
  <div class="card">
    <div class="title">
      <div class="stripesbox">
        <div class="stripes" />
      </div>
      <p class="title-content">
        <slot name="title"
          ><h1>meow</h1>
          <p>
            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
            eiusmod tempor incididunt ut labore et dolore magna aliqua. Quam
            elementum pulvinar etiam non quam lacus suspendisse faucibus
            interdum.
          </p></slot
        >
      </p>
    </div>
    <div class="content">
      <slot>
        <p>
          Accumsan sit amet nulla facilisi morbi tempus iaculis urna. Congue
          mauris rhoncus aenean vel elit scelerisque mauris pellentesque. Proin
          libero nunc consequat interdum varius sit amet. Maecenas volutpat
          blandit aliquam etiam erat. Laoreet id donec ultrices tincidunt arcu
          non sodales neque sodales. Ornare aenean euismod elementum nisi. Non
          odio euismod lacinia at quis risus sed vulputate odio. Eget arcu
          dictum varius duis at consectetur lorem. Sed risus pretium quam
          vulputate dignissim suspendisse in est ante. Tristique et egestas quis
          ipsum suspendisse ultrices gravida. Nec dui nunc mattis enim ut tellus
          elementum sagittis.
        </p>
      </slot>
    </div>
  </div>
</template>

<style scoped lang="scss">
.card {
  display: flex;
  flex-direction: column;
  background-color: var(--light-orange);
  border-radius: var(--radius);
  flex: 1;
}

.content {
  padding: var(--pad-size);
  display: flex;
  flex-direction: column;
  background-color: var(--light-orange);
  border-radius: var(--radius);
  flex: 1;
  gap: var(--pad-size);
}

.title-content {
  flex-grow: 1;
  display: block;
  margin-top: auto;
  margin-bottom: auto;
  text-align: right;
  margin: var(--pad-size);
}

.title:deep(h2) {
  color: var(--tertiary-accent);
}

.title {
  display: flex;
  margin-top: var(--fs-large);
  background-color: var(--black);
  align-items: flex-end;
}

.stripes {
  min-width: calc(var(--fs-xl) * 2);
  width: calc(var(--fs-xl) * 2);
  aspect-ratio: 1/1;
  background: linear-gradient(
    45deg,
    var(--quaternary-accent) 12.5%,
    var(--tertiary-accent) 13%,
    var(--tertiary-accent) 25.5%,
    var(--secondary-accent) 26%,
    var(--secondary-accent) 37.5%,
    var(--primary-accent) 38%,
    var(--primary-accent) 50%,
    transparent 50.5%
  );
}

@media screen and (min-width: 1280px) {
  .card {
    flex-direction: row;
    display: flex;
    box-sizing: border-box;
  }

  .title {
    align-items: flex-start;
    margin-left: var(--fs-large);
    display: flex;
    flex-direction: column-reverse;
    margin-top: 0;
    flex-grow: 1;
    width: min-content;
  }

  .title-content {
    display: flex;
    flex-direction: column;
    justify-content: center;
    margin-top: auto;
    margin-bottom: auto;
    margin-left: auto;
  }

  .content {
    flex-grow: 1.61803;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .content:deep(p) {
    font-size: var(--fs-regular);
  }
}

.title:deep(h1) {
  color: var(--quaternary-accent);
}

.title:deep(a) {
  color: var(--secondary-accent);
}

.title:deep(p) {
  color: var(--light-orange);
}

.card {
  &:deep(p) {
    font-size: var(--fs-regular);
  }
  &:deep(h1) {
    font-size: var(--fs-xl);
  }
  &:deep(*) {
    color: var(--black);
  }
}

@media (max-width: 600px) {
  .title {
    flex-direction: column-reverse;
  }

  .stripesbox {
    display: flex;
    width: 100%;
  }
}
</style>
