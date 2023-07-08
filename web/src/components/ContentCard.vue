<template>
  <div class="card">
    <div class="title">
      <div class="stripes" />
      <p class="title-content">
        <slot name="title"><h1>meow</h1></slot>
      </p>
    </div>
    <div class="content">
      <slot></slot>
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

.title {
  display: flex;
  margin-top: var(--fs-large);
  background-color: var(--black);
  align-items: flex-end;
}

.stripes {
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

@media screen and (orientation: landscape) and (min-width: 1280px) {
  .card {
    flex-direction: row;
    display: flex;
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
</style>
