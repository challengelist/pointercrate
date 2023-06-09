import {
  generateDemon,
  embedVideo,
  generatePlayer, PlayerSelectionDialog,
} from "../modules/demonlist.mjs";
import {
  FilteredPaginator,
  Viewer,
  setupDropdownEditor,
  setupFormDialogEditor,
  PaginatorEditorBackend,
  rangeOverflow,
  rangeUnderflow,
  badInput,
  stepMismatch,
  valueMissing,
  typeMismatch,
  del,
  displayError,
  Form,
  post, setupEditorDialog, FormDialog,
} from "../modules/form.mjs";

export let demonManager;

export class DemonManager extends FilteredPaginator {
  constructor(csrfToken) {
    super("demon-pagination", generateDemon, "name_contains");

    this.output = new Viewer(
      this.html.parentNode.getElementsByClassName("viewer-content")[0],
      this
    );

    this._tok = csrfToken;

    this.retrievalEndpoint = "/api/v2/demons/";

    this._id = document.getElementById("demon-demon-id");
    this._name = document.getElementById("demon-demon-name");

    this._video = document.getElementById("demon-video");
    this._video_link = document.getElementById("demon-video-link");

    this._position = document.getElementById("demon-position");
    this._fps = document.getElementById("demon-fps");

    this._verifier = document.getElementById("demon-verifier");
    this._publisher = document.getElementById("demon-publisher");

    this._creators = document.getElementById("demon-creators");

    let videoForm = setupFormDialogEditor(
      new PaginatorEditorBackend(this, csrfToken, false),
      "demon-video-dialog",
      "demon-video-pen",
      this.output
    );

    videoForm.addValidators({
      "demon-video-edit": {
        "Please enter a valid URL": typeMismatch,
      },
    });

    for (let errorCode of [42222, 42223, 42224, 42225]) {
      videoForm.addErrorOverride(errorCode, "demon-video-edit");
    }

    let positionForm = setupFormDialogEditor(
      new PaginatorEditorBackend(this, csrfToken, true),
      "demon-position-dialog",
      "demon-position-pen",
      this.output
    );
    let fpsForm = setupFormDialogEditor(
      new PaginatorEditorBackend(this, csrfToken, true),
      "demon-fps-dialog",
      "demon-fps-pen",
      this.output
    );

    positionForm.addValidators({
      "demon-position-edit": {
        "Demon position must be at least 1": rangeUnderflow,
        "Demon position must be a valid integer": badInput,
        "Demon position mustn't be a decimal": stepMismatch,
        "Please enter a position": valueMissing,
      },
    });

    positionForm.addErrorOverride(42213, "demon-position-edit");

    let nameForm = setupFormDialogEditor(
      new PaginatorEditorBackend(this, csrfToken, true),
      "demon-name-dialog",
      "demon-name-pen",
      this.output
    );

    nameForm.addValidators({
      "demon-name-edit": {
        "Please provide a name for the demon": valueMissing,
      },
    });

    
    this._hidden = setupDropdownEditor(
      new PaginatorEditorBackend(this, csrfToken, true),
      "edit-demon-hidden",
      "hidden",
      this.output,
      { true: true, false: false }
    );

    setupEditorDialog(new PlayerSelectionDialog("demon-verifier-dialog"), "demon-verifier-pen", new PaginatorEditorBackend(this, csrfToken, true), this.output, data => ({verifier: data.player}));

    setupEditorDialog(new PlayerSelectionDialog("demon-publisher-dialog"), "demon-publisher-pen", new PaginatorEditorBackend(this, csrfToken, true), this.output, data => ({publisher: data.player}));
  }

  onReceive(response) {
    super.onReceive(response);

    if (response.status == 204) {
      return;
    }

    this._id.innerText = this.currentObject.id;
    this._name.innerText = this.currentObject.name;
    this._position.innerText = this.currentObject.position;
    this._fps.innerText = this.currentObject.fps;

    var embeddedVideo = embedVideo(this.currentObject.video);

    if (embeddedVideo !== undefined) {
      this._video.style.display = "block";
      this._video_link.style.display = "initial";
      this._video.src = embeddedVideo;
    } else {
      this._video.style.display = "none";
    }

    if (this.currentObject.video) {
      this._video_link.href = this.currentObject.video;
      this._video_link.innerHTML = this.currentObject.video;
    } else {
      this._video_link.style.display = "none";
    }

    this._publisher.innerHTML =
      this.currentObject.publisher.name +
      " (" +
      this.currentObject.publisher.id +
      ")";
    this._verifier.innerHTML =
      this.currentObject.verifier.name +
      " (" +
      this.currentObject.verifier.id +
      ")";

      
    this._hidden.selectSilently(this.currentObject.hidden.toString());

    while (this._creators.lastChild) {
      this._creators.removeChild(this._creators.lastChild);
    }

    for (let creator of this.currentObject.creators) {
      this.addCreator(creator);
    }
  }

  addCreator(creator) {
    let html = insertCreatorInto(creator, this._creators);
    html.children[0].addEventListener("click", () => {
      del(
        "/api/v2/demons/" +
          this.currentObject.id +
          "/creators/" +
          creator.id +
          "/",
        {
          "X-CSRF-TOKEN": this._tok,
        }
      )
        .then(() => {
          this._creators.removeChild(html);
          this.output.setSuccess("owo uwu owo");
        })
        .catch(displayError(this.output));
    });
  }
}

function insertCreatorInto(creator, container) {
  let html = createCreatorHtml(creator);
  if (container.children.length == 0) {
    // trailing comma
    html.removeChild(html.lastChild);
  }

  container.prepend(html);
  return html;
}

function createCreatorHtml(creator) {
  let span = document.createElement("span");

  span.style.display = "inline-block"; // Prevent line breaks in the middle of a creator, especially between the 'x' and the name

  let i = document.createElement("i");

  i.innerText = creator.name;

  if (creator.id) {
    i.innerText += " (" + creator.id + ")";
  }

  let closeX = document.createElement("i");

  closeX.classList.add("fa");
  closeX.classList.add("fa-times");
  closeX.classList.add("hover");
  closeX.classList.add("fa-lg");

  closeX.style.margin = "3px";

  span.appendChild(closeX);
  span.appendChild(i);
  span.appendChild(document.createTextNode(", "));

  return span;
}

function setupDemonAdditionForm(csrfToken) {
  let form = new Form(document.getElementById("demon-submission-form"));


  let verifierSelector = new PlayerSelectionDialog("demon-add-verifier-dialog");
  document.getElementById("demon-add-verifier-pen").addEventListener('click', () => verifierSelector.open().then(data => form.input('demon-add-verifier').value = data.player));

  let publisherSelector = new PlayerSelectionDialog("demon-add-publisher-dialog");
  document.getElementById("demon-add-publisher-pen").addEventListener('click', () => publisherSelector.open().then(data => form.input('demon-add-publisher').value = data.player));


  form.addValidators({
    "demon-add-name": { "Please specify a name": valueMissing },
    "demon-add-position": {
      "Please specify a position": valueMissing,
      "Demon position cannot be smaller than 1": rangeUnderflow,
      "Demon position must be a valid integer": badInput,
      "Demon position must be integer": stepMismatch,
    },
    "demon-add-verifier": { "Please specify a verifier": valueMissing },
    "demon-add-publisher": { "Please specify a publisher": valueMissing },
    "demon-add-video": { "Please enter a valid URL": typeMismatch },
  });

  form.creators = [];

  form.onSubmit(() => {
    let data = form.serialize();

    data["creators"] = form.creators;
    data["requirement"] = 100;

    post("/api/v1/demons/", { "X-CSRF-TOKEN": csrfToken }, data)
      .then(() => {
        form.setSuccess("Successfully added demon!");
        demonManager.refresh();
        form.clear();
      })
      .catch(displayError(form));
  });

  return form;
}

export function initialize(csrfToken) {
  demonManager = new DemonManager(csrfToken);
  demonManager.initialize();

  let addDemonForm = setupDemonAdditionForm(csrfToken);

  let creatorFormDialog = new PlayerSelectionDialog("demon-add-creator-dialog");
  let dialogCreators = document.getElementById("demon-add-creators");

  let button1 = document.getElementById("demon-add-creator-pen");
  let button2 = document.getElementById("add-demon-add-creator-pen");

  button1.addEventListener("click", () => {
    creatorFormDialog.submissionPredicateFactory = (data) => {
      return post(
          "/api/v2/demons/" + demonManager.currentObject.id + "/creators/",
          {
            "X-CSRF-TOKEN": csrfToken,
          },
          {creator: data.player}
      )
          .then((response) => {
            let location = response.headers["location"];

            demonManager.addCreator({
              name: data.player,
              id: location.substring(
                  location.lastIndexOf("/", location.length - 2) + 1,
                  location.length - 1
              ),
            });

            demonManager.output.setSuccess("Successfully added creator");
          })
          .catch(response => {
            displayError(creatorFormDialog.form)(response);
            throw response;
          });
    }
    creatorFormDialog.open();
  });

  button2.addEventListener("click", () => {
    creatorFormDialog.submissionPredicateFactory = (data) => new Promise(resolve => resolve({creator: data.player}));
    creatorFormDialog.open()
        .then(data => {
          let creator = insertCreatorInto({ name: data.creator }, dialogCreators);
          creator.children[0].addEventListener("click", () => {
            addDemonForm.creators.splice(
                addDemonForm.creators.indexOf(data.creator),
                1
            );
            dialogCreators.removeChild(creator);
          });

          addDemonForm.creators.push(data.creator);
        });
  });
}
