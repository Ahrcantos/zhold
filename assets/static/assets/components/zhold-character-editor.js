class CharacterEditor extends HTMLElement {
  constructor() {
    super();
  }

  connectedCallback() {
    const shadow = this.attachShadow({ mode: 'open' });

    const div = document.createElement('div');
    div.textContent = 'Editor';
    shadow.appendChild(div);
  }
}

customElements.define("zhold-character-editor", CharacterEditor);
